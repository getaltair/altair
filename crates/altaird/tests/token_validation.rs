//! Token validation, end to end, against a provider that exists only here.
//!
//! Every key is generated in-process and every token is minted here, so the
//! suite runs offline and nothing resembling a private key is ever committed.
//! Time is injected rather than slept through: an expiry test that sleeps is
//! a test that fails on a loaded CI runner.

use std::net::SocketAddr;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use altaird::auth::{
    Authentication, Authenticator, Clock, Fault, HttpJwks, IssuerConfig, KeyCache, bearer_token,
};
use altaird::testing::TestDb;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use ring::rand::SystemRandom;
use ring::signature::{ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair, KeyPair};
use serde_json::{Value, json};
use sqlx::postgres::PgPool;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use uuid::Uuid;

const ISSUER: &str = "https://auth.example.test/application/o/altair/";
const AUDIENCE: &str = "altair";

/// A fixed "now" the tests move around. Far enough from zero that a token can
/// expire without the arithmetic going negative.
const NOW: i64 = 1_780_000_000;

// ---------------------------------------------------------------------------
// A signing key, generated per test
// ---------------------------------------------------------------------------

struct TestKey {
    kid: String,
    encoding: EncodingKey,
    x: String,
    y: String,
}

fn generate(kid: &str) -> TestKey {
    let rng = SystemRandom::new();
    let pkcs8 = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng)
        .expect("generate a key pair");
    let pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8.as_ref(), &rng)
        .expect("read the key pair back");

    // Uncompressed SEC1 point: 0x04 || X || Y, 32 bytes each on P-256.
    let point = pair.public_key().as_ref();
    assert_eq!(point.len(), 65, "unexpected public point encoding");

    TestKey {
        kid: kid.to_owned(),
        encoding: EncodingKey::from_ec_der(pkcs8.as_ref()),
        x: URL_SAFE_NO_PAD.encode(&point[1..33]),
        y: URL_SAFE_NO_PAD.encode(&point[33..65]),
    }
}

impl TestKey {
    fn jwk(&self) -> Value {
        json!({
            "kty": "EC",
            "crv": "P-256",
            "alg": "ES256",
            "use": "sig",
            "kid": self.kid,
            "x": self.x,
            "y": self.y,
        })
    }

    /// Mint a token, optionally lying about which key signed it.
    fn mint_as(&self, kid: &str, claims: &Value) -> String {
        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(kid.to_owned());
        jsonwebtoken::encode(&header, claims, &self.encoding).expect("mint")
    }

    fn mint(&self, claims: &Value) -> String {
        self.mint_as(&self.kid, claims)
    }
}

fn jwks(keys: &[&TestKey]) -> String {
    json!({ "keys": keys.iter().map(|k| k.jwk()).collect::<Vec<_>>() }).to_string()
}

fn claims(subject: &str, exp: i64) -> Value {
    json!({ "iss": ISSUER, "aud": AUDIENCE, "sub": subject, "iat": exp - 3600, "exp": exp })
}

// ---------------------------------------------------------------------------
// A provider that can be taken away
// ---------------------------------------------------------------------------

struct FakeProvider {
    addr: SocketAddr,
    body: Arc<Mutex<String>>,
    fetches: Arc<AtomicUsize>,
    task: Option<JoinHandle<()>>,
}

impl FakeProvider {
    async fn serving(body: String) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let body = Arc::new(Mutex::new(body));
        let fetches = Arc::new(AtomicUsize::new(0));

        let (b, f) = (Arc::clone(&body), Arc::clone(&fetches));
        let task = tokio::spawn(async move {
            while let Ok((mut socket, _)) = listener.accept().await {
                let (b, f) = (Arc::clone(&b), Arc::clone(&f));
                tokio::spawn(async move {
                    let mut request = Vec::new();
                    let mut chunk = [0u8; 512];
                    while !request.windows(4).any(|w| w == b"\r\n\r\n") {
                        match socket.read(&mut chunk).await {
                            Ok(0) | Err(_) => return,
                            Ok(n) => request.extend_from_slice(&chunk[..n]),
                        }
                    }
                    f.fetch_add(1, Ordering::SeqCst);
                    let payload = b.lock().expect("body").clone();
                    let response = format!(
                        "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\n\
                         content-length: {}\r\nconnection: close\r\n\r\n{payload}",
                        payload.len()
                    );
                    let _ = socket.write_all(response.as_bytes()).await;
                    let _ = socket.shutdown().await;
                });
            }
        });

        Self {
            addr,
            body,
            fetches,
            task: Some(task),
        }
    }

    fn uri(&self) -> String {
        format!("http://{}/jwks", self.addr)
    }

    fn now_serving(&self, body: String) {
        *self.body.lock().expect("body") = body;
    }

    fn fetches(&self) -> usize {
        self.fetches.load(Ordering::SeqCst)
    }

    /// Take the provider away. The port stops accepting, so the next fetch
    /// fails the way a restarted Authentik does.
    async fn go_away(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
            let _ = task.await;
        }
    }
}

impl Drop for FakeProvider {
    fn drop(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
        }
    }
}

// ---------------------------------------------------------------------------
// Injected time
// ---------------------------------------------------------------------------

struct TestClock(AtomicI64);

impl TestClock {
    fn at(seconds: i64) -> Arc<Self> {
        Arc::new(Self(AtomicI64::new(seconds)))
    }

    fn advance(&self, seconds: i64) {
        self.0.fetch_add(seconds, Ordering::SeqCst);
    }
}

impl Clock for TestClock {
    fn now_unix(&self) -> i64 {
        self.0.load(Ordering::SeqCst)
    }
}

// ---------------------------------------------------------------------------
// Assembly
// ---------------------------------------------------------------------------

fn authenticator(
    pool: PgPool,
    jwks_uri: &str,
    clock: Arc<TestClock>,
    refresh_after: Duration,
    min_refetch: Duration,
) -> Authenticator {
    let source = Arc::new(HttpJwks::new(jwks_uri).expect("build the jwks client"));
    let keys = KeyCache::new(source).with_policy(refresh_after, min_refetch);
    let config = IssuerConfig {
        issuer: ISSUER.to_owned(),
        audience: AUDIENCE.to_owned(),
    };
    Authenticator::new(config, keys, pool).with_clock(clock)
}

/// Default policy for tests that are not about caching: never treat a cached
/// set as fresh, never rate-limit. Every call is allowed to reach the
/// provider, which is the least forgiving setting.
fn eager(pool: PgPool, jwks_uri: &str, clock: Arc<TestClock>) -> Authenticator {
    authenticator(pool, jwks_uri, clock, Duration::ZERO, Duration::ZERO)
}

async fn seed_household(pool: &PgPool) -> Uuid {
    let id = Uuid::new_v4();
    sqlx::query("INSERT INTO household (id, name, created_at) VALUES ($1, 'test', now())")
        .bind(id)
        .execute(pool)
        .await
        .expect("seed household");
    id
}

async fn seed_member(pool: &PgPool, household: Uuid, subject: &str, departed: bool) -> Uuid {
    let id = Uuid::new_v4();
    let sql = if departed {
        "INSERT INTO membership \
         (id, household_id, subject, display_name, administrator, joined_at, departed_at) \
         VALUES ($1, $2, $3, 'Test', false, now(), now())"
    } else {
        "INSERT INTO membership \
         (id, household_id, subject, display_name, administrator, joined_at, departed_at) \
         VALUES ($1, $2, $3, 'Test', false, now(), NULL)"
    };
    sqlx::query(sql)
        .bind(id)
        .bind(household)
        .bind(subject)
        .execute(pool)
        .await
        .expect("seed membership");
    id
}

// ---------------------------------------------------------------------------
// Done when: a valid token resolves to a membership
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_valid_token_resolves_to_a_membership() {
    let db = TestDb::new().await;
    let household = seed_household(&db.pool).await;
    let membership = seed_member(&db.pool, household, "sub-alice", false).await;

    let key = generate("key-1");
    let provider = FakeProvider::serving(jwks(&[&key])).await;
    let auth = eager(db.pool.clone(), &provider.uri(), TestClock::at(NOW));

    let token = key.mint(&claims("sub-alice", NOW + 3600));
    let outcome = auth.authenticate(Some(&token)).await.expect("no fault");

    let member = outcome.member().expect("a membership");
    assert_eq!(member.membership_id(), membership);
    assert_eq!(member.household_id(), household);
    assert_eq!(member.subject(), "sub-alice");
    assert!(!member.is_administrator());
}

#[tokio::test]
async fn a_bearer_header_carries_the_credential() {
    let db = TestDb::new().await;
    let household = seed_household(&db.pool).await;
    seed_member(&db.pool, household, "sub-alice", false).await;

    let key = generate("key-1");
    let provider = FakeProvider::serving(jwks(&[&key])).await;
    let auth = eager(db.pool.clone(), &provider.uri(), TestClock::at(NOW));

    let token = key.mint(&claims("sub-alice", NOW + 3600));
    let header = format!("bearer {token}");
    let outcome = auth
        .authenticate(bearer_token(Some(&header)))
        .await
        .expect("no fault");

    assert!(
        outcome.member().is_some(),
        "a lowercase scheme is still bearer"
    );
}

// ---------------------------------------------------------------------------
// Done when: an expired token produces the wait outcome
// ---------------------------------------------------------------------------

#[tokio::test]
async fn an_expired_token_is_the_wait_outcome() {
    let db = TestDb::new().await;
    let household = seed_household(&db.pool).await;
    seed_member(&db.pool, household, "sub-alice", false).await;

    let key = generate("key-1");
    let provider = FakeProvider::serving(jwks(&[&key])).await;
    let clock = TestClock::at(NOW);
    let auth = eager(db.pool.clone(), &provider.uri(), Arc::clone(&clock));

    let token = key.mint(&claims("sub-alice", NOW + 3600));
    assert!(
        auth.authenticate(Some(&token))
            .await
            .expect("no fault")
            .member()
            .is_some(),
        "the token is good before it expires"
    );

    // Past expiry, and past the skew allowance. No sleeping.
    clock.advance(3600 + 120);

    let outcome = auth.authenticate(Some(&token)).await.expect("no fault");
    assert_eq!(
        outcome,
        Authentication::Unauthenticated,
        "an expired session is a wait, and a wait is an outcome rather than an error"
    );
}

#[tokio::test]
async fn a_token_within_the_skew_allowance_still_resolves() {
    let db = TestDb::new().await;
    let household = seed_household(&db.pool).await;
    seed_member(&db.pool, household, "sub-alice", false).await;

    let key = generate("key-1");
    let provider = FakeProvider::serving(jwks(&[&key])).await;
    let clock = TestClock::at(NOW);
    let auth = eager(db.pool.clone(), &provider.uri(), Arc::clone(&clock));

    let token = key.mint(&claims("sub-alice", NOW));
    clock.advance(30);

    assert!(
        auth.authenticate(Some(&token))
            .await
            .expect("no fault")
            .member()
            .is_some(),
        "two machines disagreeing by half a minute is not an expired session"
    );
}

#[tokio::test]
async fn a_token_that_is_not_yet_valid_is_the_wait_outcome() {
    let db = TestDb::new().await;
    let household = seed_household(&db.pool).await;
    seed_member(&db.pool, household, "sub-alice", false).await;

    let key = generate("key-1");
    let provider = FakeProvider::serving(jwks(&[&key])).await;
    let auth = eager(db.pool.clone(), &provider.uri(), TestClock::at(NOW));

    let mut c = claims("sub-alice", NOW + 7200);
    c["nbf"] = json!(NOW + 3600);
    let token = key.mint(&c);

    assert_eq!(
        auth.authenticate(Some(&token)).await.expect("no fault"),
        Authentication::Unauthenticated
    );
}

#[tokio::test]
async fn an_absent_credential_is_the_wait_outcome() {
    let db = TestDb::new().await;
    let key = generate("key-1");
    let provider = FakeProvider::serving(jwks(&[&key])).await;
    let auth = eager(db.pool.clone(), &provider.uri(), TestClock::at(NOW));

    assert_eq!(
        auth.authenticate(None).await.expect("no fault"),
        Authentication::Unauthenticated
    );
    assert_eq!(
        auth.authenticate(Some("   ")).await.expect("no fault"),
        Authentication::Unauthenticated
    );
    assert_eq!(
        provider.fetches(),
        0,
        "no credential means no reason to ask the provider anything"
    );
}

// ---------------------------------------------------------------------------
// Done when: a forged token produces nothing
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_forged_token_produces_nothing() {
    let db = TestDb::new().await;
    let household = seed_household(&db.pool).await;
    seed_member(&db.pool, household, "sub-alice", false).await;

    let published = generate("key-1");
    let attacker = generate("key-1-impostor");
    let provider = FakeProvider::serving(jwks(&[&published])).await;
    let auth = eager(db.pool.clone(), &provider.uri(), TestClock::at(NOW));

    // Signed by a key the provider never published, but claiming the kid of
    // one it did. Everything except the signature is impeccable.
    let forged = attacker.mint_as("key-1", &claims("sub-alice", NOW + 3600));

    let outcome = auth.authenticate(Some(&forged)).await.expect("no fault");
    assert_eq!(outcome, Authentication::Unauthenticated);
    assert!(outcome.member().is_none(), "no membership from a forgery");
}

#[tokio::test]
async fn a_forgery_and_an_expiry_are_the_same_value() {
    let db = TestDb::new().await;
    let household = seed_household(&db.pool).await;
    seed_member(&db.pool, household, "sub-alice", false).await;

    let published = generate("key-1");
    let attacker = generate("key-1-impostor");
    let provider = FakeProvider::serving(jwks(&[&published])).await;
    let clock = TestClock::at(NOW);
    let auth = eager(db.pool.clone(), &provider.uri(), Arc::clone(&clock));

    let expired = published.mint(&claims("sub-alice", NOW - 3600));
    let forged = attacker.mint_as("key-1", &claims("sub-alice", NOW + 3600));

    let from_expiry = auth.authenticate(Some(&expired)).await.expect("no fault");
    let from_forgery = auth.authenticate(Some(&forged)).await.expect("no fault");

    assert_eq!(
        from_expiry, from_forgery,
        "nothing distinguishes which half of validation failed, here or on the wire"
    );
}

#[tokio::test]
async fn a_token_shaped_like_nonsense_produces_nothing() {
    let db = TestDb::new().await;
    let key = generate("key-1");
    let provider = FakeProvider::serving(jwks(&[&key])).await;
    let auth = eager(db.pool.clone(), &provider.uri(), TestClock::at(NOW));

    for garbage in ["not a token", "a.b.c", "...", "eyJhbGciOiJub25lIn0..", ""] {
        assert_eq!(
            auth.authenticate(Some(garbage)).await.expect("no fault"),
            Authentication::Unauthenticated,
            "{garbage:?} produced something"
        );
    }
}

#[tokio::test]
async fn a_symmetric_signature_over_the_public_key_produces_nothing() {
    let db = TestDb::new().await;
    let household = seed_household(&db.pool).await;
    seed_member(&db.pool, household, "sub-alice", false).await;

    let key = generate("key-1");
    let provider = FakeProvider::serving(jwks(&[&key])).await;
    let auth = eager(db.pool.clone(), &provider.uri(), TestClock::at(NOW));

    // The algorithm-confusion attack: present the published public key back
    // as though it were an HMAC secret.
    let secret = format!("{}{}", key.x, key.y);
    let mut header = Header::new(Algorithm::HS256);
    header.kid = Some("key-1".to_owned());
    let token = jsonwebtoken::encode(
        &header,
        &claims("sub-alice", NOW + 3600),
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("mint");

    assert_eq!(
        auth.authenticate(Some(&token)).await.expect("no fault"),
        Authentication::Unauthenticated
    );
}

#[tokio::test]
async fn another_issuer_and_another_audience_produce_nothing() {
    let db = TestDb::new().await;
    let household = seed_household(&db.pool).await;
    seed_member(&db.pool, household, "sub-alice", false).await;

    let key = generate("key-1");
    let provider = FakeProvider::serving(jwks(&[&key])).await;
    let auth = eager(db.pool.clone(), &provider.uri(), TestClock::at(NOW));

    let mut wrong_issuer = claims("sub-alice", NOW + 3600);
    wrong_issuer["iss"] = json!("https://elsewhere.example.test/");
    let mut wrong_audience = claims("sub-alice", NOW + 3600);
    wrong_audience["aud"] = json!("something-else");

    for c in [wrong_issuer, wrong_audience] {
        assert_eq!(
            auth.authenticate(Some(&key.mint(&c)))
                .await
                .expect("no fault"),
            Authentication::Unauthenticated
        );
    }
}

// ---------------------------------------------------------------------------
// Claim to membership
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_subject_with_no_membership_produces_nothing_and_creates_nothing() {
    let db = TestDb::new().await;
    let household = seed_household(&db.pool).await;
    seed_member(&db.pool, household, "sub-alice", false).await;

    let key = generate("key-1");
    let provider = FakeProvider::serving(jwks(&[&key])).await;
    let auth = eager(db.pool.clone(), &provider.uri(), TestClock::at(NOW));

    let token = key.mint(&claims("sub-a-stranger", NOW + 3600));
    assert_eq!(
        auth.authenticate(Some(&token)).await.expect("no fault"),
        Authentication::Unauthenticated
    );

    let members: i64 = sqlx::query_scalar("SELECT count(*) FROM membership")
        .fetch_one(&db.pool)
        .await
        .expect("count");
    assert_eq!(
        members, 1,
        "an unknown subject is an outcome, never an invitation to create a membership"
    );
}

#[tokio::test]
async fn a_departed_membership_does_not_resolve() {
    let db = TestDb::new().await;
    let household = seed_household(&db.pool).await;
    let departed = seed_member(&db.pool, household, "sub-departed", true).await;

    let key = generate("key-1");
    let provider = FakeProvider::serving(jwks(&[&key])).await;
    let auth = eager(db.pool.clone(), &provider.uri(), TestClock::at(NOW));

    let token = key.mint(&claims("sub-departed", NOW + 3600));
    assert_eq!(
        auth.authenticate(Some(&token)).await.expect("no fault"),
        Authentication::Unauthenticated,
        "departure withdraws the ability to act, whatever references survive"
    );

    // The row itself is untouched: references to it survive departure.
    let still_there: i64 = sqlx::query_scalar("SELECT count(*) FROM membership WHERE id = $1")
        .bind(departed)
        .fetch_one(&db.pool)
        .await
        .expect("count");
    assert_eq!(still_there, 1);
}

#[tokio::test]
async fn an_administrator_is_reported_as_one() {
    let db = TestDb::new().await;
    let household = seed_household(&db.pool).await;
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO membership \
         (id, household_id, subject, administrator, joined_at) \
         VALUES ($1, $2, 'sub-admin', true, now())",
    )
    .bind(id)
    .bind(household)
    .execute(&db.pool)
    .await
    .expect("seed");

    let key = generate("key-1");
    let provider = FakeProvider::serving(jwks(&[&key])).await;
    let auth = eager(db.pool.clone(), &provider.uri(), TestClock::at(NOW));

    let token = key.mint(&claims("sub-admin", NOW + 3600));
    let outcome = auth.authenticate(Some(&token)).await.expect("no fault");
    assert!(outcome.member().expect("a membership").is_administrator());
}

// ---------------------------------------------------------------------------
// Unauthenticated reaches no query surface
// ---------------------------------------------------------------------------

/// The type system already makes this hard to get wrong — nothing can build a
/// `Member` except a successful validation — but a type argument is not a
/// runnable test. This one puts a store that cannot answer behind the
/// validator: a forged token must still produce the wait outcome, while a
/// valid one must fault. The pair is only explicable if the forged token
/// never reached the store.
#[tokio::test]
async fn an_unvalidated_credential_never_reaches_the_store() {
    let db = TestDb::new().await;
    let household = seed_household(&db.pool).await;
    seed_member(&db.pool, household, "sub-alice", false).await;

    let published = generate("key-1");
    let attacker = generate("key-1-impostor");
    let provider = FakeProvider::serving(jwks(&[&published])).await;
    let auth = authenticator(
        db.pool.clone(),
        &provider.uri(),
        TestClock::at(NOW),
        Duration::from_secs(3600),
        Duration::ZERO,
    );

    let good = published.mint(&claims("sub-alice", NOW + 3600));
    let forged = attacker.mint_as("key-1", &claims("sub-alice", NOW + 3600));

    // Prime the key cache while everything still works.
    assert!(
        auth.authenticate(Some(&good))
            .await
            .expect("no fault")
            .member()
            .is_some()
    );

    db.pool.close().await;

    assert_eq!(
        auth.authenticate(Some(&forged)).await.expect("no fault"),
        Authentication::Unauthenticated,
        "a forgery is answered without the store being consulted"
    );

    match auth.authenticate(Some(&good)).await {
        Err(Fault::Store(_)) => {}
        other => panic!("a valid token should have reached the store: {other:?}"),
    }
}

/// The other half of the same claim: the outcome of a bad credential is never
/// on the error channel, so a caller cannot confuse a wait with a fault.
#[tokio::test]
async fn no_property_of_a_token_produces_an_error() {
    let db = TestDb::new().await;
    let household = seed_household(&db.pool).await;
    seed_member(&db.pool, household, "sub-alice", false).await;

    let published = generate("key-1");
    let attacker = generate("key-1-impostor");
    let provider = FakeProvider::serving(jwks(&[&published])).await;
    let clock = TestClock::at(NOW);
    let auth = eager(db.pool.clone(), &provider.uri(), Arc::clone(&clock));

    let mut wrong_issuer = claims("sub-alice", NOW + 3600);
    wrong_issuer["iss"] = json!("https://elsewhere.example.test/");

    let candidates = vec![
        published.mint(&claims("sub-alice", NOW - 86_400)),
        published.mint(&claims("sub-nobody", NOW + 3600)),
        published.mint(&wrong_issuer),
        published.mint_as("key-unknown", &claims("sub-alice", NOW + 3600)),
        attacker.mint_as("key-1", &claims("sub-alice", NOW + 3600)),
        "garbage".to_owned(),
    ];

    for token in candidates {
        let outcome = auth.authenticate(Some(&token)).await;
        assert!(
            matches!(outcome, Ok(Authentication::Unauthenticated)),
            "a token property reached the error channel"
        );
    }
}

// ---------------------------------------------------------------------------
// Key caching
// ---------------------------------------------------------------------------

#[tokio::test]
async fn cached_keys_survive_the_provider_being_unreachable() {
    let db = TestDb::new().await;
    let household = seed_household(&db.pool).await;
    seed_member(&db.pool, household, "sub-alice", false).await;

    let key = generate("key-1");
    let mut provider = FakeProvider::serving(jwks(&[&key])).await;
    // The least forgiving policy: the cached set is stale on every call, so
    // every call tries the provider and every call must survive it failing.
    let auth = eager(db.pool.clone(), &provider.uri(), TestClock::at(NOW));

    let token = key.mint(&claims("sub-alice", NOW + 3600));
    assert!(
        auth.authenticate(Some(&token))
            .await
            .expect("no fault")
            .member()
            .is_some(),
        "primed while the provider was up"
    );
    assert_eq!(provider.fetches(), 1);

    provider.go_away().await;

    for attempt in 0..3 {
        let outcome = auth.authenticate(Some(&token)).await.expect("no fault");
        assert!(
            outcome.member().is_some(),
            "attempt {attempt}: an Authentik restart must not turn every session off"
        );
    }
}

#[tokio::test]
async fn a_cold_start_with_no_keys_is_the_wait_outcome() {
    let db = TestDb::new().await;
    let household = seed_household(&db.pool).await;
    seed_member(&db.pool, household, "sub-alice", false).await;

    let key = generate("key-1");
    let mut provider = FakeProvider::serving(jwks(&[&key])).await;
    let uri = provider.uri();
    provider.go_away().await;

    let auth = eager(db.pool.clone(), &uri, TestClock::at(NOW));
    let token = key.mint(&claims("sub-alice", NOW + 3600));

    assert_eq!(
        auth.authenticate(Some(&token)).await.expect("no fault"),
        Authentication::Unauthenticated,
        "never having reached the provider is a wait, not a fault"
    );
}

#[tokio::test]
async fn an_unseen_key_id_prompts_a_refresh() {
    let db = TestDb::new().await;
    let household = seed_household(&db.pool).await;
    seed_member(&db.pool, household, "sub-alice", false).await;

    let old = generate("key-1");
    let new = generate("key-2");
    let provider = FakeProvider::serving(jwks(&[&old])).await;
    // Cached keys stay fresh for an hour, so only an unseen kid can be what
    // sends this to the provider a second time.
    let auth = authenticator(
        db.pool.clone(),
        &provider.uri(),
        TestClock::at(NOW),
        Duration::from_secs(3600),
        Duration::ZERO,
    );

    let before = old.mint(&claims("sub-alice", NOW + 3600));
    assert!(
        auth.authenticate(Some(&before))
            .await
            .expect("no fault")
            .member()
            .is_some()
    );
    assert_eq!(provider.fetches(), 1);

    provider.now_serving(jwks(&[&new]));

    let after = new.mint(&claims("sub-alice", NOW + 3600));
    assert!(
        auth.authenticate(Some(&after))
            .await
            .expect("no fault")
            .member()
            .is_some(),
        "a rotation is noticed by the kid arriving, without polling"
    );
    assert_eq!(provider.fetches(), 2);
}

#[tokio::test]
async fn an_unseen_key_id_with_the_provider_away_produces_nothing() {
    let db = TestDb::new().await;
    let household = seed_household(&db.pool).await;
    seed_member(&db.pool, household, "sub-alice", false).await;

    let old = generate("key-1");
    let new = generate("key-2");
    let mut provider = FakeProvider::serving(jwks(&[&old])).await;
    let auth = authenticator(
        db.pool.clone(),
        &provider.uri(),
        TestClock::at(NOW),
        Duration::from_secs(3600),
        Duration::ZERO,
    );

    let known = old.mint(&claims("sub-alice", NOW + 3600));
    assert!(
        auth.authenticate(Some(&known))
            .await
            .expect("no fault")
            .member()
            .is_some()
    );

    provider.go_away().await;

    let unknown = new.mint(&claims("sub-alice", NOW + 3600));
    assert_eq!(
        auth.authenticate(Some(&unknown)).await.expect("no fault"),
        Authentication::Unauthenticated,
        "a key that cannot be fetched is a wait, not a fault"
    );
    assert!(
        auth.authenticate(Some(&known))
            .await
            .expect("no fault")
            .member()
            .is_some(),
        "and the keys already held are untouched by the failed refresh"
    );
}

#[tokio::test]
async fn fetches_are_rate_limited_against_a_stream_of_unknown_key_ids() {
    let db = TestDb::new().await;
    let household = seed_household(&db.pool).await;
    seed_member(&db.pool, household, "sub-alice", false).await;

    let key = generate("key-1");
    let provider = FakeProvider::serving(jwks(&[&key])).await;
    let auth = authenticator(
        db.pool.clone(),
        &provider.uri(),
        TestClock::at(NOW),
        Duration::from_secs(3600),
        Duration::from_secs(60),
    );

    for n in 0..50 {
        let token = key.mint_as(&format!("invented-{n}"), &claims("sub-alice", NOW + 3600));
        assert_eq!(
            auth.authenticate(Some(&token)).await.expect("no fault"),
            Authentication::Unauthenticated
        );
    }

    assert_eq!(
        provider.fetches(),
        1,
        "anyone who can send a header could otherwise point this instance at its own provider"
    );
}

// ---------------------------------------------------------------------------
// Header parsing
// ---------------------------------------------------------------------------

#[test]
fn bearer_tokens_are_read_out_of_a_header() {
    assert_eq!(bearer_token(Some("Bearer abc")), Some("abc"));
    assert_eq!(bearer_token(Some("bearer abc")), Some("abc"));
    assert_eq!(bearer_token(Some("BEARER  abc  ")), Some("abc"));
    assert_eq!(bearer_token(None), None);
    assert_eq!(bearer_token(Some("")), None);
    assert_eq!(bearer_token(Some("abc")), None);
    assert_eq!(bearer_token(Some("Basic abc")), None);
    assert_eq!(bearer_token(Some("Bearer ")), None);
}
