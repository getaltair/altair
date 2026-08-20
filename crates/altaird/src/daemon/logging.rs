//! What this instance may write down about itself, decided before any layer
//! was switched on.
//!
//! # The policy
//!
//! **The daemon logs its own lifecycle and its own faults. It logs nothing
//! about a request.**
//!
//! Permitted: that it started, what it is listening on, which preconditions it
//! checked, that a background task started or stopped, that a store or object
//! store operation failed and the error the library gave, and that it is
//! shutting down.
//!
//! Never: a request, a response, a query's text, an entity's title, a
//! statement, a bound argument, a token, a subject, a membership identity, a
//! path within the object store, or the database URL. Not at any level, not
//! behind a flag, and not in a span's fields — a span that names the member is
//! a record of who asked, which is the same disclosure as one that names what
//! they asked for.
//!
//! # Why the rule is "nothing about a request" and not "nothing sensitive"
//!
//! The read path is required to keep **no record of what was asked**. That is
//! not a rule about secrets, it is a rule about there being no trail; a log
//! line saying "member 3f2a queried" keeps the record even with the text
//! stripped out, and one saying "Query answered 4 results in 12ms" keeps it
//! for anyone holding the timestamps. A per-request log line that is safe is
//! hard to tell from one that is not, and the difference is a code review away
//! from being lost. A blanket rule needs no judgement at the call site and
//! fails loudly in review: any logging macro in `read/` or in the served
//! surface is wrong on its face, and `tests/logging.rs` says so.
//!
//! # Two defences, and only one of them is a guarantee
//!
//! sqlx logs every statement — at `DEBUG` ordinarily, and at `WARN` with the
//! whole SQL attached once one crosses a second, which a write waiting on the
//! change sequence row does routinely. That is a stated invariant broken by a
//! library default, silently, somewhere nobody thinks to audit.
//!
//! 1. **At the source.** [`crate::store::connect`] calls
//!    `disable_statement_logging`, so nothing is ever emitted to log. This is
//!    the guarantee: it cannot be undone by an environment variable.
//! 2. **At the subscriber.** The default filter below silences the same target
//!    anyway. This is defence in depth and nothing more — it is exactly the
//!    kind of protection somebody removes at 2am to chase an unrelated bug,
//!    which is why it is not the one relied on.
//!
//! `tests/logging.rs` induces the slow-statement path deliberately, so the
//! check is watched failing on a real emission rather than passing over a
//! condition that never arose.
//!
//! # The default filter
//!
//! `RUST_LOG` overrides it, because an operator debugging their own instance
//! is entitled to. That is safe for the part that matters: nothing on the
//! request paths calls a logging macro at any level, and defence (1) means
//! raising a level cannot bring a person's statement back.
//!
//! **What raising it does bring back is worth knowing.**
//! `sqlx_postgres::connection::resolve` prints its own catalog SQL at `TRACE`,
//! under a target that `disable_statement_logging` does not govern. It carries
//! nothing anybody asked for — it is the driver looking up type OIDs — but it
//! is the reason this filter holds everything below the instance at `warn`
//! rather than naming one target and trusting the rest. An operator who widens
//! it is choosing that, which is different from it happening by default.

use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::util::SubscriberInitExt;

/// What is recorded when nothing says otherwise.
///
/// The instance at `INFO`, everything else at `WARN`, and sqlx's statement
/// target off outright. Libraries below this crate are noisy about connections
/// and streams in a way that says nothing about whether the instance is well,
/// and a log a person cannot read is a log nobody reads.
pub const DEFAULT_FILTER: &str = "warn,altaird=info,sqlx::query=off";

/// Install the subscriber, writing to standard error.
///
/// Idempotent in the sense that matters: a second call, or a call in a process
/// where something else installed one first, is reported rather than a panic.
///
/// # Errors
///
/// If a global subscriber is already installed.
pub fn install() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    install_with(std::io::stderr)
}

/// The same policy, writing somewhere else.
///
/// **This exists so the test asserts the real policy** rather than a
/// reconstruction of it that could agree with itself while the daemon does
/// something different. `tests/logging.rs` hands it a buffer and greps what
/// the daemon actually produced.
///
/// # Errors
///
/// If a global subscriber is already installed.
pub fn install_with<W>(writer: W) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    W: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let filter =
        std::env::var(EnvFilter::DEFAULT_ENV).unwrap_or_else(|_| DEFAULT_FILTER.to_owned());
    install_with_filter(writer, &filter)
}

/// The same again, with the filter stated rather than taken from the
/// environment.
///
/// **This is how the guarantee is tested.** With the default filter in place,
/// a test asserting that no statement reaches the log would pass whether or
/// not `store::connect` disabled statement logging, because the filter alone
/// silences the target — a check that cannot fail is a check that is not
/// there. `tests/logging.rs` calls this with the silencing removed, so what it
/// exercises is the defence that cannot be turned off by configuration.
///
/// It is also the honest model of an operator at two in the morning turning
/// levels up to chase something unrelated. That is the situation the source
/// side exists for.
///
/// # Errors
///
/// If a global subscriber is already installed.
pub fn install_with_filter<W>(
    writer: W,
    filter: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    W: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(filter))
        .with_writer(writer)
        // The target is a module path in this crate, which says which part of
        // the instance spoke and nothing about who it spoke for. It is also
        // what makes the filter above legible to whoever has to override it.
        .with_target(true)
        .finish()
        .try_init()
        .map_err(Into::into)
}
