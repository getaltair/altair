//! Where background work attaches, how it is cancelled, and how shutdown
//! waits for it.
//!
//! **Nothing runs here yet, and that is the point.** The derivation worker is
//! Wave 5's and reclamation is Wave 2.4's; both become tasks in this process.
//! Deciding their shape now costs a small file. Deciding it later means
//! deciding it twice — once for whichever lands first, and again when the
//! second one wants something the first shape cannot give it — with a
//! half-written worker already depending on the answer.
//!
//! # The three decisions
//!
//! **Where they attach.** To the daemon, beside the server and not inside it.
//! A task is spawned by [`Tasks::spawn`] before the server begins serving, and
//! it holds whatever it needs — the pool, the object store — by clone. Nothing
//! about a background task is reachable from a request, and nothing about a
//! request is reachable from one.
//!
//! **How they are cancelled.** One signal, broadcast, shared with the server.
//! [`Tasks::signal`] hands out a [`Shutdown`] that resolves when the daemon is
//! stopping; the same value is what the gRPC server is given as its shutdown
//! future. So there is exactly one thing that means "we are stopping", and a
//! task cannot be cancelled while the server carries on, or the reverse.
//!
//! A task **selects** on it rather than being aborted at an arbitrary await
//! point. Reclamation deletes bytes and the derivation worker claims rows with
//! `SKIP LOCKED`; an abort between two steps of either is how a body goes
//! missing or a queue row stays claimed by nobody. Cancellation is a request,
//! and the task decides where it is safe to honour it.
//!
//! **How shutdown waits.** [`Tasks::stop`] signals, then joins every task with
//! one deadline over all of them, then reports by name any that did not
//! finish. It does not wait forever: a daemon that will not exit is worse for
//! an operator than one that says what it gave up on. It does not abort
//! silently either — a task that overran is named, because a worker regularly
//! overrunning shutdown is a bug in the worker and the log line is the only
//! thing that would ever say so.
//!
//! # Deliberately not here
//!
//! No scheduling, no intervals, no restart-on-panic, no supervision tree. Each
//! of those is a decision the first real task should make with its own
//! requirements in front of it, and a framework written before the work it
//! frames is a framework the work then has to argue with.

use std::time::Duration;

use tokio::sync::watch;
use tokio::task::JoinHandle;

/// How long every background task together gets to finish once asked to stop.
///
/// One budget for all of them rather than one each, because it bounds what an
/// operator waits for: two tasks that each take four seconds must not make
/// shutdown take eight. A task that needs longer than this to reach a safe
/// point should be doing less between safe points.
const STOP_DEADLINE: Duration = Duration::from_secs(5);

/// The daemon is stopping.
///
/// Cheap to clone; every holder is woken by the same signal. Awaiting
/// [`Shutdown::requested`] on a fresh one that has already been signalled
/// returns immediately, so a task spawned during shutdown does not run
/// forever.
#[derive(Clone, Debug)]
pub struct Shutdown(watch::Receiver<bool>);

impl Shutdown {
    /// Resolves when shutdown has been asked for.
    ///
    /// Also resolves if the sender is gone, which means the daemon that owned
    /// it has been dropped — an outcome a task should treat exactly as it
    /// treats being asked to stop.
    pub async fn requested(&mut self) {
        // Already stopping: do not wait for a change that has been and gone.
        if *self.0.borrow() {
            return;
        }
        let _ = self.0.changed().await;
    }
}

/// The background tasks this process is running, and the one signal that stops
/// them.
pub struct Tasks {
    signal: watch::Sender<bool>,
    running: Vec<(&'static str, JoinHandle<()>)>,
}

impl Default for Tasks {
    fn default() -> Self {
        Self::new()
    }
}

impl Tasks {
    #[must_use]
    pub fn new() -> Self {
        let (signal, _) = watch::channel(false);
        Self {
            signal,
            running: Vec::new(),
        }
    }

    /// A handle on "the daemon is stopping", for a task or for the server.
    #[must_use]
    pub fn signal(&self) -> Shutdown {
        Shutdown(self.signal.subscribe())
    }

    /// Start a task, handing it the shutdown signal.
    ///
    /// The name is what [`Tasks::stop`] reports if it overruns, so it should
    /// read as the name of the work rather than of the function.
    pub fn spawn<F, Fut>(&mut self, name: &'static str, task: F)
    where
        F: FnOnce(Shutdown) -> Fut,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let handle = tokio::spawn(task(self.signal()));
        self.running.push((name, handle));
    }

    /// How many tasks are running. For a test, and for nothing else.
    #[must_use]
    pub fn count(&self) -> usize {
        self.running.len()
    }

    /// Ask every task to stop, and wait up to [`STOP_DEADLINE`] for all of
    /// them together.
    ///
    /// Answers with the names of the tasks that did not finish in time and
    /// were abandoned. An empty answer is the ordinary one.
    pub async fn stop(self) -> Vec<&'static str> {
        // Ignored: no receiver left means no task left to tell.
        let _ = self.signal.send(true);

        let mut overran = Vec::new();
        let deadline = tokio::time::Instant::now() + STOP_DEADLINE;

        for (name, handle) in self.running {
            match tokio::time::timeout_at(deadline, handle).await {
                // Finished, or finished by panicking. A panic in a background
                // task is the task's own business to report; shutdown's job is
                // only to know it is no longer running.
                Ok(_) => {}
                Err(_) => overran.push(name),
            }
        }
        overran
    }
}
