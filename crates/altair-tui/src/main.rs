//! The terminal client.
//!
//! Two modes. Ordinarily it is the person's client; with the adapter flag it is
//! the conformance adapter, which is the same store and the same outbox behind
//! a line of JSON instead of a screen.

use std::sync::Arc;

use altair_tui::app::App;
use altair_tui::config::Config;
use altair_tui::replica::Replica;
use altair_tui::sender::Sender;
use altair_tui::signals::Signals;

#[cfg(feature = "conformance")]
use altair_tui::adapter;

#[tokio::main]
async fn main() -> std::process::ExitCode {
    #[cfg(feature = "conformance")]
    if std::env::args().any(|argument| argument == adapter::FLAG) {
        adapter::run().await;
        return std::process::ExitCode::SUCCESS;
    }

    let config = Config::from_environment();
    let signals = Arc::new(Signals::new());

    // **Both of these are started before anything is drawn and neither is
    // waited for.** Sending and reading are the ordinary path; nothing the
    // person does is behind either, and an instance that is not there costs
    // them nothing at all.
    if let Some(instance) = config.instance.as_deref() {
        match Sender::new(&config.state_dir, instance, Arc::clone(&signals)) {
            Ok(sender) => drop(tokio::spawn(sender.run())),
            Err(error) => eprintln!("altair: nothing will send: {error}"),
        }
        match Replica::new(&config.state_dir, instance, Arc::clone(&signals)) {
            Ok(replica) => drop(tokio::spawn(replica.run())),
            Err(error) => eprintln!("altair: nothing will arrive: {error}"),
        }
    }

    // A store that will not open is the one condition that reaches the
    // guarantee rather than delaying it, so it is said plainly and the client
    // does not pretend to be usable.
    let app = match App::new(&config, signals) {
        Ok(app) => app,
        Err(error) => {
            eprintln!(
                "altair: {} cannot hold your captures: {error}",
                config.state_dir.display()
            );
            return std::process::ExitCode::FAILURE;
        }
    };

    // The screens are drawn on a blocking thread. The terminal is a blocking
    // thing and the two background tasks are not; keeping them apart is what
    // makes a stalled send something the person never notices.
    let drawn = tokio::task::spawn_blocking(move || {
        let mut terminal = ratatui::init();
        let outcome = app.run(&mut terminal);
        ratatui::restore();
        outcome
    })
    .await;

    match drawn {
        Ok(Ok(())) => std::process::ExitCode::SUCCESS,
        Ok(Err(error)) => {
            eprintln!("altair: {error}");
            std::process::ExitCode::FAILURE
        }
        Err(error) => {
            eprintln!("altair: {error}");
            std::process::ExitCode::FAILURE
        }
    }
}
