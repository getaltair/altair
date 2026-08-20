//! Token validation and membership resolution.
//!
//! Governed by DR-005: **the instance validates; it does not authenticate.**
//! It fetches and caches the provider's signing keys, verifies a signature,
//! and reads claims. There are no passwords here, no account creation, no
//! recovery, and no user list. A subject claim that matches no membership is
//! an outcome, never an invitation to create one.
//!
//! # The outcome is data, not an error
//!
//! [`Authenticator::authenticate`] returns `Result<Authentication, Fault>`,
//! and the two channels mean sharply different things:
//!
//! * [`Authentication`] is what the *credential* produced. Every property of
//!   a token — absent, malformed, expired, not yet valid, wrong issuer, wrong
//!   audience, unverifiable signature, a subject this instance holds no
//!   membership for — lands in [`Authentication::Unauthenticated`]. Per
//!   DR-005 that is a **wait**: the outbox holds, nothing is dropped, nothing
//!   signals, and the ordinary path clears it by continuing to run.
//! * [`Fault`] is the instance describing *itself*. The only thing that can
//!   produce one is the structured store being unavailable while resolving a
//!   membership. **No property of a token can produce an `Err`.** That is the
//!   whole point: a caller that matches on the outcome cannot accidentally
//!   fold an expired session in with a genuine failure, which is how a
//!   capture disappears while the interface stays silent.
//!
//! A refusal — the third outcome the substrate names, and the one that
//! signals — is deliberately *not* modelled here. A refusal is a statement
//! about a request, and this boundary never sees a request; a request that is
//! refused was authenticated first. Refusal belongs to the write path.
//!
//! # Unauthenticated reaches no query surface
//!
//! This is a type-level guarantee rather than a convention. [`Member`] has
//! private fields and no public constructor: the only code in the crate that
//! can build one is [`Authenticator::authenticate`], on the branch where a
//! signature verified and a claim resolved. Any surface that needs a member
//! takes a `&Member`, and an unauthenticated request cannot produce the value
//! required to call it. (`Member::for_test` exists behind the `testing`
//! feature, which the binary never enables.)
//!
//! # Forged and expired are the same value
//!
//! `Unauthenticated` carries no reason, so a forged token and an expired one
//! are literally indistinguishable to a caller and on the wire. Three things
//! in the documents force this:
//!
//! 1. DR-005's obligation is unconditional — *"a request whose token fails
//!    validation is unauthenticated and reaches no query surface"*. A forged
//!    signature is a validation failure, so it is unauthenticated, and DR-005
//!    defines unauthenticated as the wait.
//! 2. The design refuses to leak elsewhere on exactly this pattern: nothing
//!    distinguishes an entity that does not exist from one the member cannot
//!    see, including the gRPC status. Telling a caller *which half* of
//!    validation failed is the same class of disclosure.
//! 3. The benign cause of a signature failure is self-clearing. A provider
//!    that rotated its keys leaves honest clients holding tokens signed by a
//!    key that no longer verifies, and their ordinary refresh fixes it. If
//!    that signalled, a routine rotation would raise a fault at a person —
//!    a wrong signal, which the substrate says costs attention. Silence costs
//!    an attacker nothing, because an attacker already knows what they sent.
//!
//! So "a forged token produces nothing" means exactly that: no membership, no
//! reason, no information — the same value an expired one produces.

mod identify;
mod jwks;

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::Deserialize;
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub use identify::{Identify, Identity};
pub use jwks::{HttpJwks, JwksSource, JwksUnavailable, KeyCache};

/// Signature algorithms this instance will consider.
///
/// Asymmetric only. A JWKS entry is an RSA or EC public key, and an HMAC
/// algorithm arriving in a header would ask the instance to verify with a
/// public key used as a shared secret — the classic confusion. `jsonwebtoken`
/// rejects the family mismatch on its own; naming the set here means such a
/// header never reaches that check.
const PERMITTED_ALGORITHMS: &[Algorithm] = &[
    Algorithm::RS256,
    Algorithm::RS384,
    Algorithm::RS512,
    Algorithm::PS256,
    Algorithm::PS384,
    Algorithm::PS512,
    Algorithm::ES256,
    Algorithm::ES384,
];

/// Tolerance for clock skew between this instance and the provider.
///
/// Not configurable. Token lifetimes are Authentik's configuration per
/// DR-005; this is not a lifetime, it is an allowance for two machines
/// disagreeing about the second.
const SKEW_LEEWAY_SECONDS: i64 = 60;

/// A validated member identity.
///
/// Constructible only by [`Authenticator::authenticate`]. Holding one is
/// proof that a signature verified against the provider's keys and that the
/// subject claim resolved to a membership that has not departed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    membership_id: Uuid,
    household_id: Uuid,
    subject: String,
    administrator: bool,
}

impl Member {
    /// The internal identity. This is what authorship, assignment, and
    /// audience reference — never the subject.
    #[must_use]
    pub fn membership_id(&self) -> Uuid {
        self.membership_id
    }

    #[must_use]
    pub fn household_id(&self) -> Uuid {
        self.household_id
    }

    /// The external key the token carried. Diagnostics only; nothing
    /// downstream should key on it.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    #[must_use]
    pub fn is_administrator(&self) -> bool {
        self.administrator
    }

    /// Build a member without a token, for tests in other lanes.
    ///
    /// Behind the `testing` feature, which the binary never enables, so the
    /// unforgeability of `Member` holds in every shipped build.
    #[cfg(feature = "testing")]
    #[must_use]
    pub fn for_test(
        membership_id: Uuid,
        household_id: Uuid,
        subject: impl Into<String>,
        administrator: bool,
    ) -> Self {
        Self {
            membership_id,
            household_id,
            subject: subject.into(),
            administrator,
        }
    }
}

/// What a presented credential produced.
///
/// Exhaustive by construction. See the module documentation for why there is
/// no third variant and why `Unauthenticated` carries no reason.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub enum Authentication {
    /// The signature verified and the subject named a living membership.
    Member(Member),
    /// No usable claim. **A wait, not a fault**: nothing signals, the outbox
    /// holds, and the ordinary path clears it by continuing to run.
    Unauthenticated,
}

impl Authentication {
    #[must_use]
    pub fn member(&self) -> Option<&Member> {
        match self {
            Self::Member(m) => Some(m),
            Self::Unauthenticated => None,
        }
    }

    #[must_use]
    pub fn is_unauthenticated(&self) -> bool {
        matches!(self, Self::Unauthenticated)
    }
}

/// The instance failing, never the credential failing.
#[derive(Debug)]
pub enum Fault {
    /// The structured store could not be reached or read.
    Store(sqlx::Error),
}

impl std::fmt::Display for Fault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Store(e) => write!(f, "the structured store was unavailable: {e}"),
        }
    }
}

impl std::error::Error for Fault {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Store(e) => Some(e),
        }
    }
}

/// Where "now" comes from, so expiry is testable without sleeping.
pub trait Clock: Send + Sync + 'static {
    /// Seconds since the Unix epoch.
    fn now_unix(&self) -> i64;
}

/// The wall clock.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now_unix(&self) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX))
    }
}

/// The one issuer this instance accepts, and the audience it requires.
///
/// Whether the instance ever accepts more than one issuer is deliberately not
/// decided (DR-005), so there is no machinery for a second here.
#[derive(Debug, Clone)]
pub struct IssuerConfig {
    pub issuer: String,
    pub audience: String,
}

/// Turns a presented credential into an [`Authentication`].
pub struct Authenticator {
    config: IssuerConfig,
    keys: KeyCache,
    pool: PgPool,
    clock: Arc<dyn Clock>,
}

/// The claims this instance reads. Everything else the provider sends is
/// ignored — the instance owns no user list and has no use for a profile.
#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
    nbf: Option<i64>,
}

impl Authenticator {
    pub fn new(config: IssuerConfig, keys: KeyCache, pool: PgPool) -> Self {
        Self {
            config,
            keys,
            pool,
            clock: Arc::new(SystemClock),
        }
    }

    /// Replace the clock. Tests move time; nothing else should.
    #[must_use]
    pub fn with_clock(mut self, clock: Arc<dyn Clock>) -> Self {
        self.clock = clock;
        self
    }

    /// Validate a credential and resolve it to a membership.
    ///
    /// `credential` is the bare token, as produced by [`bearer_token`].
    /// `None` — no credential at all — is [`Authentication::Unauthenticated`]
    /// without touching the key cache or the store.
    ///
    /// # Errors
    ///
    /// Only [`Fault::Store`], and only once a signature has already verified.
    pub async fn authenticate(&self, credential: Option<&str>) -> Result<Authentication, Fault> {
        let Some(token) = credential.map(str::trim).filter(|t| !t.is_empty()) else {
            return Ok(Authentication::Unauthenticated);
        };

        let Some(claims) = self.verified_claims(token).await else {
            return Ok(Authentication::Unauthenticated);
        };

        self.resolve(&claims.sub).await
    }

    /// Signature, issuer, audience, and time. `None` for any failure, with no
    /// record of which — see the module documentation.
    async fn verified_claims(&self, token: &str) -> Option<Claims> {
        let header = decode_header(token).ok()?;
        if !PERMITTED_ALGORITHMS.contains(&header.alg) {
            return None;
        }

        let jwk = self.keys.key_for(header.kid.as_deref()).await?;

        // If the provider declared an algorithm for this key, the header must
        // agree with it. A key is published for one purpose.
        if let Some(declared) = jwk.common.key_algorithm
            && !declares(declared, header.alg)
        {
            return None;
        }

        let key = DecodingKey::from_jwk(&jwk).ok()?;

        let mut validation = Validation::new(header.alg);
        validation.set_issuer(&[self.config.issuer.as_str()]);
        validation.set_audience(&[self.config.audience.as_str()]);
        // `exp` is demanded, but compared against the injected clock below
        // rather than against `SystemTime::now()` buried in the library —
        // otherwise expiry could only be tested by sleeping.
        validation.set_required_spec_claims(&["exp", "iss", "aud", "sub"]);
        validation.validate_exp = false;
        validation.validate_nbf = false;

        let claims = decode::<Claims>(token, &key, &validation).ok()?.claims;

        let now = self.clock.now_unix();
        if claims.exp.saturating_add(SKEW_LEEWAY_SECONDS) < now {
            return None;
        }
        if let Some(nbf) = claims.nbf
            && nbf.saturating_sub(SKEW_LEEWAY_SECONDS) > now
        {
            return None;
        }

        Some(claims)
    }

    /// The subject is the external key onto the membership, which is the
    /// internal thing (DR-005).
    ///
    /// A departed membership does not resolve. The schema's note that
    /// "references to this membership survive it" is about rows that already
    /// point at it — authorship never changes and an audience entry stays
    /// where it is — not about the person still being able to act. Departure
    /// is the household withdrawing someone, and audience enforcement built
    /// on member identity would be meaningless if it did not take effect.
    /// A departed subject produces the same nothing an unknown subject does,
    /// because saying which would tell someone outside the household about
    /// its state.
    async fn resolve(&self, subject: &str) -> Result<Authentication, Fault> {
        let row = sqlx::query(
            "SELECT id, household_id, administrator \
             FROM membership \
             WHERE subject = $1 AND departed_at IS NULL",
        )
        .bind(subject)
        .fetch_optional(&self.pool)
        .await
        .map_err(Fault::Store)?;

        let Some(row) = row else {
            return Ok(Authentication::Unauthenticated);
        };

        Ok(Authentication::Member(Member {
            membership_id: row.try_get("id").map_err(Fault::Store)?,
            household_id: row.try_get("household_id").map_err(Fault::Store)?,
            subject: subject.to_owned(),
            administrator: row.try_get("administrator").map_err(Fault::Store)?,
        }))
    }
}

/// Whether a JWK's declared algorithm is the one a header claimed.
///
/// Written out rather than derived from a string, because `jsonwebtoken`
/// models the two enumerations separately and anything that maps them through
/// their text is a rename away from silently accepting a mismatch. Anything
/// outside [`PERMITTED_ALGORITHMS`] is false by construction.
fn declares(declared: jsonwebtoken::jwk::KeyAlgorithm, alg: Algorithm) -> bool {
    use jsonwebtoken::jwk::KeyAlgorithm as Declared;

    matches!(
        (declared, alg),
        (Declared::RS256, Algorithm::RS256)
            | (Declared::RS384, Algorithm::RS384)
            | (Declared::RS512, Algorithm::RS512)
            | (Declared::PS256, Algorithm::PS256)
            | (Declared::PS384, Algorithm::PS384)
            | (Declared::PS512, Algorithm::PS512)
            | (Declared::ES256, Algorithm::ES256)
            | (Declared::ES384, Algorithm::ES384)
    )
}

/// The bare token out of an `authorization` header value.
///
/// Tolerates any casing of the scheme, which is what RFC 9110 requires and
/// what clients on four platforms would otherwise get wrong in four ways.
#[must_use]
pub fn bearer_token(authorization: Option<&str>) -> Option<&str> {
    let value = authorization?.trim();
    let (scheme, token) = value.split_once(' ')?;
    if !scheme.eq_ignore_ascii_case("bearer") {
        return None;
    }
    let token = token.trim();
    (!token.is_empty()).then_some(token)
}
