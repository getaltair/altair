//! The provider's signing keys: where they come from, and how long a copy
//! stays good.
//!
//! # The policy, and why it is this one
//!
//! **A cached key set has no expiry.** It goes stale — after
//! [`DEFAULT_REFRESH_AFTER`] the cache will try for a newer copy — but stale
//! is not empty. A failed refresh leaves the previous set in place and
//! validation carries on against it. This is what "key caching survives the
//! provider being briefly unreachable" means: an Authentik restart, a
//! certificate renewal, or a container reschedule must not turn every
//! household member's session off.
//!
//! **A key the cache has not seen is what actually triggers a fetch.** Key
//! rotation shows up as a `kid` in a header that the cached set does not
//! contain, so an unseen `kid` prompts one refresh attempt and the token is
//! then validated against whatever came back. That is rotation handled
//! without polling.
//!
//! **Attempts are rate-limited** by [`DEFAULT_MIN_REFETCH_INTERVAL`], because
//! the unseen-`kid` trigger is reachable by anyone who can send a request.
//! Without the limit, a stream of forged headers each carrying a fresh random
//! `kid` would turn the instance into a load generator pointed at its own
//! identity provider.
//!
//! **A refresh that fails is not a fault and signals nothing.** No outcome
//! here is an error: `key_for` returns `None`, the caller reads that as
//! unauthenticated, and unauthenticated is a wait. A provider that is briefly
//! away, a cold start before the first successful fetch, and an unseen `kid`
//! that the refresh did not produce are all the same silence, and all of them
//! clear by the ordinary path continuing to run.

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use http_body_util::{BodyExt, Empty, Limited};
use hyper::body::Bytes;
use hyper::header::ACCEPT;
use hyper::{Method, Request, Uri};
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use jsonwebtoken::jwk::{Jwk, JwkSet, PublicKeyUse};

/// How long a fetched key set is used before the cache tries for a newer one.
pub const DEFAULT_REFRESH_AFTER: Duration = Duration::from_secs(15 * 60);

/// The floor between two fetch attempts, however many requests ask for a key
/// the cache does not hold.
pub const DEFAULT_MIN_REFETCH_INTERVAL: Duration = Duration::from_secs(60);

/// How long a single fetch may take before it is abandoned.
const FETCH_TIMEOUT: Duration = Duration::from_secs(5);

/// The largest document the instance will read from the provider.
const MAX_JWKS_BYTES: usize = 1 << 20;

/// The provider could not be asked, or did not answer usefully.
///
/// Deliberately opaque and deliberately not an `Error` any caller handles:
/// this never escapes the cache. It exists so a failed fetch can be told from
/// a successful one internally, and nothing more.
#[derive(Debug)]
pub struct JwksUnavailable(String);

impl std::fmt::Display for JwksUnavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the signing keys could not be fetched: {}", self.0)
    }
}

impl std::error::Error for JwksUnavailable {}

impl JwksUnavailable {
    fn new(reason: impl std::fmt::Display) -> Self {
        Self(reason.to_string())
    }
}

type JwksFuture<'a> = Pin<Box<dyn Future<Output = Result<JwkSet, JwksUnavailable>> + Send + 'a>>;

/// Where a key set comes from.
///
/// A trait so tests can stand up a provider without a network, and so an
/// operator's deployment could later supply keys from a file without the
/// validation path knowing.
pub trait JwksSource: Send + Sync + 'static {
    fn fetch(&self) -> JwksFuture<'_>;
}

/// The provider's JWKS endpoint over HTTP.
///
/// Uses rustls with the ring provider, which is the one crypto stack already
/// in this binary — `sqlx` brings the same one, and `jsonwebtoken` verifies
/// signatures with it too.
pub struct HttpJwks {
    uri: Uri,
    client: Client<hyper_rustls::HttpsConnector<HttpConnector>, Empty<Bytes>>,
}

impl HttpJwks {
    /// # Errors
    ///
    /// If the URI does not parse, or no trust store can be loaded. Both are
    /// misconfiguration found at startup, not conditions a request meets.
    pub fn new(jwks_uri: &str) -> anyhow::Result<Self> {
        let uri: Uri = jwks_uri.parse()?;

        let builder = hyper_rustls::HttpsConnectorBuilder::new();
        // The operator's own CA is the ordinary case for a self-hosted
        // provider, so the system trust store is tried first. Bundled roots
        // are the fallback for an image that ships without one.
        let connector = match builder.with_native_roots() {
            Ok(b) => b.https_or_http().enable_http1().build(),
            Err(_) => hyper_rustls::HttpsConnectorBuilder::new()
                .with_webpki_roots()
                .https_or_http()
                .enable_http1()
                .build(),
        };

        let client = Client::builder(TokioExecutor::new()).build(connector);
        Ok(Self { uri, client })
    }

    async fn get(&self) -> Result<JwkSet, JwksUnavailable> {
        let request = Request::builder()
            .method(Method::GET)
            .uri(self.uri.clone())
            .header(ACCEPT, "application/json")
            .body(Empty::<Bytes>::new())
            .map_err(JwksUnavailable::new)?;

        let response = tokio::time::timeout(FETCH_TIMEOUT, self.client.request(request))
            .await
            .map_err(|_| JwksUnavailable::new("the request timed out"))?
            .map_err(JwksUnavailable::new)?;

        if !response.status().is_success() {
            return Err(JwksUnavailable::new(format!(
                "the provider answered {}",
                response.status()
            )));
        }

        let body = Limited::new(response.into_body(), MAX_JWKS_BYTES);
        let bytes = tokio::time::timeout(FETCH_TIMEOUT, body.collect())
            .await
            .map_err(|_| JwksUnavailable::new("the body timed out"))?
            .map_err(|e| JwksUnavailable::new(e.to_string()))?
            .to_bytes();

        serde_json::from_slice(&bytes).map_err(JwksUnavailable::new)
    }
}

impl JwksSource for HttpJwks {
    fn fetch(&self) -> JwksFuture<'_> {
        Box::pin(self.get())
    }
}

#[derive(Default)]
struct State {
    keys: Option<Arc<JwkSet>>,
    fetched_at: Option<Instant>,
    last_attempt: Option<Instant>,
}

/// The cached copy of the provider's keys.
pub struct KeyCache {
    source: Arc<dyn JwksSource>,
    state: Mutex<State>,
    refresh_after: Duration,
    min_refetch_interval: Duration,
}

impl KeyCache {
    #[must_use]
    pub fn new(source: Arc<dyn JwksSource>) -> Self {
        Self {
            source,
            state: Mutex::new(State::default()),
            refresh_after: DEFAULT_REFRESH_AFTER,
            min_refetch_interval: DEFAULT_MIN_REFETCH_INTERVAL,
        }
    }

    /// Override the refresh policy. Tests compress it; nothing else should.
    #[must_use]
    pub fn with_policy(mut self, refresh_after: Duration, min_refetch_interval: Duration) -> Self {
        self.refresh_after = refresh_after;
        self.min_refetch_interval = min_refetch_interval;
        self
    }

    /// The key a header named, if this instance can produce it.
    ///
    /// `None` where the header carried no `kid` and the set holds more than
    /// one key: guessing which key signed a token is how a revoked key stays
    /// useful. A set holding exactly one key needs no `kid`, which keeps a
    /// minimal provider working.
    pub async fn key_for(&self, kid: Option<&str>) -> Option<Jwk> {
        let (found, stale, may_attempt) = {
            let state = self.lock();
            let now = Instant::now();
            let found = state.keys.as_deref().and_then(|set| select(set, kid));
            let stale = state
                .fetched_at
                .is_none_or(|at| now.duration_since(at) >= self.refresh_after);
            let may_attempt = state
                .last_attempt
                .is_none_or(|at| now.duration_since(at) >= self.min_refetch_interval);
            (found, stale, may_attempt)
        };

        // The common path: a key we hold, from a copy that is still fresh.
        // No network, no lock held across an await.
        if found.is_some() && !stale {
            return found;
        }

        if may_attempt {
            self.refresh().await;
        } else {
            return found;
        }

        let state = self.lock();
        // A refresh that failed left the previous set in place, so this is
        // still the survive-an-outage path.
        state
            .keys
            .as_deref()
            .and_then(|set| select(set, kid))
            .or(found)
    }

    async fn refresh(&self) {
        let fetched = self.source.fetch().await;
        let now = Instant::now();

        let mut state = self.lock();
        state.last_attempt = Some(now);
        if let Ok(set) = fetched {
            state.keys = Some(Arc::new(set));
            state.fetched_at = Some(now);
        }
        // On failure: keys, and the time they were fetched, are left exactly
        // as they were. The provider being away is not a reason to forget
        // what it already told us.
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, State> {
        // A panic while holding this lock would poison it; the state behind
        // it is a cache, so continuing with it is strictly better than
        // turning every session off.
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }
}

/// Pick the key a header named, ignoring any key the provider published for
/// encryption rather than signing.
fn select(set: &JwkSet, kid: Option<&str>) -> Option<Jwk> {
    let signing = || {
        set.keys
            .iter()
            .filter(|k| !matches!(k.common.public_key_use, Some(PublicKeyUse::Encryption)))
    };

    match kid {
        Some(kid) => signing()
            .find(|k| k.common.key_id.as_deref() == Some(kid))
            .cloned(),
        None => {
            let mut only = signing();
            let first = only.next()?;
            only.next().is_none().then(|| first.clone())
        }
    }
}
