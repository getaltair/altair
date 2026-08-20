pub mod auth;
pub mod body;
pub mod daemon;
pub mod objects;
pub mod read;
pub mod service;
pub mod store;
pub mod write;

#[cfg(feature = "testing")]
pub mod testing;

/// The process. See [`daemon`] for what starting one involves and in what
/// order.
///
/// # Errors
///
/// If the instance is not configured, a precondition is not met, or the server
/// fails.
pub fn run() -> anyhow::Result<()> {
    daemon::run()
}
