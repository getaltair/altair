//! Everything the daemon must be told, and nothing it can work out itself.
//!
//! # Six values, no defaults
//!
//! **Nothing here falls back.** A default database URL points at somebody
//! else's database, a default object root puts a household's files somewhere
//! nobody chose, and a default listen address is the one that matters: guess
//! `0.0.0.0` and a self-hosted instance is on every interface the machine has,
//! including the one facing the street. The failure mode of an absent value is
//! a process that will not start and says which value it wanted. The failure
//! mode of a guessed one is silent and can be a disclosure.
//!
//! So there is exactly one constructor from the environment, it names every
//! variable, and an absent or unusable one is refused.
//!
//! # Every problem at once
//!
//! [`Config::from_env`] collects every complaint before reporting any of them.
//! An operator standing up an instance for the first time has all six wrong;
//! finding that out one restart at a time is six restarts.
//!
//! # What is deliberately not here
//!
//! * **No discovery.** The JWKS URI is given rather than fetched from the
//!   issuer's `.well-known` document, because discovery is a network call and
//!   this is startup. `auth::jwks` is explicit that a provider being briefly
//!   away is a wait that clears by the ordinary path continuing to run — an
//!   instance that refused to start because Authentik was still booting would
//!   convert that wait into an outage, and a household restarting both
//!   together is the ordinary case rather than a strange one. **Nothing in
//!   startup contacts the identity provider.**
//! * **No token lifetimes, no key rotation interval, no clock skew.** Those
//!   are the provider's configuration (DR-005) or constants in `auth`, and an
//!   operator turning a knob here would be overriding a decision made
//!   somewhere with the reasoning attached.
//! * **No pool size.** See [`crate::store`] on why a larger pool would not
//!   help.
//! * **No log level.** `RUST_LOG` is read by the subscriber (see
//!   [`crate::daemon::logging`]) and is not part of the instance's
//!   configuration, because what may be logged is a decision and not a
//!   setting.

use std::net::SocketAddr;

/// Where the structured store is.
pub const DATABASE_URL: &str = "ALTAIR_DATABASE_URL";
/// The directory the object store owns, whole.
pub const OBJECT_ROOT: &str = "ALTAIR_OBJECT_ROOT";
/// The `iss` claim this instance accepts, and only this one.
pub const ISSUER: &str = "ALTAIR_ISSUER";
/// The `aud` claim this instance requires.
pub const AUDIENCE: &str = "ALTAIR_AUDIENCE";
/// Where the issuer publishes its signing keys.
pub const JWKS_URI: &str = "ALTAIR_JWKS_URI";
/// The address to serve gRPC on. No default; see the module docs.
pub const LISTEN: &str = "ALTAIR_LISTEN";

/// Every variable the daemon reads, in the order [`Config::from_env`]
/// complains about them. Public so a test can assert the two agree, and so
/// there is one place to read the answer to "what do I have to set".
pub const VARIABLES: &[&str] = &[
    DATABASE_URL,
    OBJECT_ROOT,
    ISSUER,
    AUDIENCE,
    JWKS_URI,
    LISTEN,
];

/// The daemon's configuration.
///
/// Built from the environment in production and by hand in tests, which is why
/// the fields are public and `from_env` is only one way in.
#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    /// **A `String` and not a path type, deliberately.** DR-003 makes the
    /// object store's four operations the whole boundary and
    /// `tests/object_store_boundary.rs` fails on a filesystem type named
    /// outside `objects`. The root is the one path that crosses into the
    /// instance, it crosses here, and it is handed straight to
    /// `FilesystemObjectStore::open` without anything in between ever holding
    /// it as a path.
    pub object_root: String,
    pub issuer: String,
    pub audience: String,
    pub jwks_uri: String,
    pub listen: SocketAddr,
}

/// **Written by hand, and the database URL is redacted.** It carries a
/// password. A derived `Debug` would put it into any log line, panic message,
/// or error chain that ever formatted a `Config`, which is precisely the class
/// of accident this wave's logging policy exists to prevent — and the derive
/// would have been silent about it.
impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("database_url", &"<redacted>")
            .field("object_root", &self.object_root)
            .field("issuer", &self.issuer)
            .field("audience", &self.audience)
            .field("jwks_uri", &self.jwks_uri)
            .field("listen", &self.listen)
            .finish()
    }
}

/// What was missing or unusable, all of it.
#[derive(Debug)]
pub struct Missing(Vec<String>);

impl std::fmt::Display for Missing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the instance is not configured:")?;
        for complaint in &self.0 {
            write!(f, "\n  - {complaint}")?;
        }
        Ok(())
    }
}

impl std::error::Error for Missing {}

impl Missing {
    /// The complaints, for a test that wants to assert on them rather than on
    /// a rendered paragraph.
    #[must_use]
    pub fn complaints(&self) -> &[String] {
        &self.0
    }
}

impl Config {
    /// Read all six, or say which of them could not be read.
    ///
    /// # Errors
    ///
    /// [`Missing`], carrying one complaint per variable that was absent, empty,
    /// or unusable. Never partial: a configuration with one problem produces no
    /// `Config` at all.
    pub fn from_env() -> Result<Self, Missing> {
        let mut complaints = Vec::new();

        let mut required = |name: &str| match std::env::var(name) {
            Ok(value) if !value.trim().is_empty() => Some(value.trim().to_owned()),
            _ => {
                complaints.push(format!("{name} is not set"));
                None
            }
        };

        let database_url = required(DATABASE_URL);
        let object_root = required(OBJECT_ROOT);
        let issuer = required(ISSUER);
        let audience = required(AUDIENCE);
        let jwks_uri = required(JWKS_URI);
        let listen = required(LISTEN);

        // Parsed here rather than at bind time. An address that does not parse
        // is misconfiguration, and finding it after the pool is connected and
        // the migrations are applied is finding it later than necessary.
        let listen = listen.and_then(|value| match value.parse::<SocketAddr>() {
            Ok(addr) => Some(addr),
            Err(_) => {
                complaints.push(format!(
                    "{LISTEN} is not an address and port: {value:?}. \
                     Write it as 127.0.0.1:50051 or [::1]:50051."
                ));
                None
            }
        });

        if !complaints.is_empty() {
            return Err(Missing(complaints));
        }

        Ok(Self {
            database_url: database_url.expect("no complaint was recorded"),
            object_root: object_root.expect("no complaint was recorded"),
            issuer: issuer.expect("no complaint was recorded"),
            audience: audience.expect("no complaint was recorded"),
            jwks_uri: jwks_uri.expect("no complaint was recorded"),
            listen: listen.expect("no complaint was recorded"),
        })
    }
}
