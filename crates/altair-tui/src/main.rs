//! The terminal client.
//!
//! Two modes. Ordinarily it is the person's client; with [`adapter::FLAG`] it
//! is the conformance adapter, which is the same store and the same outbox
//! behind a line of JSON instead of a screen.

#[cfg(feature = "conformance")]
use altair_tui::adapter;

#[tokio::main]
async fn main() {
    #[cfg(feature = "conformance")]
    if std::env::args().any(|argument| argument == adapter::FLAG) {
        adapter::run().await;
        return;
    }

    // Wave 4.2's stages five and six. Until then the binary exists so that the
    // outbox has somewhere to be driven from.
    eprintln!("altair: the screens land in Wave 4.2");
}
