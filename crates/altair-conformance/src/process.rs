//! Driving a client that runs as its own process.
//!
//! One channel, newline delimited JSON, lockstep. An [`Action`] goes in on the
//! child's stdin; one [`Reply`] comes back on its stdout. Anything the child
//! writes to stderr is left alone so a failing client can still say why.
//!
//! A Kotlin transcription of this file is a `ProcessBuilder`, a
//! `BufferedWriter`, a reader thread, and a `BlockingQueue` with a poll
//! timeout. There is nothing else in it on purpose.

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::time::Duration;

use crate::client::{Action, ClientEnvironment, ClientError, ClientProcess, Reply};

/// How long the harness waits for one answer before calling the client silent.
///
/// Generous, because a silent client is a harness fault rather than a verdict.
/// E1 measures latency itself and does not lean on this.
pub const PATIENCE: Duration = Duration::from_secs(10);

/// Environment variable names handed to a launched client.
///
/// Named here rather than inline so the Kotlin harness and the Rust one cannot
/// drift, and so a client author has one list to read.
pub mod env_var {
    /// Where the instance answers, as a URL.
    pub const INSTANCE: &str = "ALTAIR_CONFORMANCE_INSTANCE";
    /// Durable storage. Survives a kill and a device restart.
    pub const STATE_DIR: &str = "ALTAIR_CONFORMANCE_STATE_DIR";
    /// Ephemeral storage. Wiped by a device restart.
    pub const RUNTIME_DIR: &str = "ALTAIR_CONFORMANCE_RUNTIME_DIR";
    /// The token to present.
    pub const TOKEN: &str = "ALTAIR_CONFORMANCE_TOKEN";
}

/// A client under test running as a child process.
pub struct ChildClient {
    child: Child,
    stdin: Option<std::process::ChildStdin>,
    replies: Receiver<Result<String, String>>,
    dead: bool,
}

impl ChildClient {
    /// Spawn `program` with `arguments`, handing it the environment.
    ///
    /// # Errors
    ///
    /// Fails if the process cannot be started or its pipes cannot be taken.
    pub fn spawn(
        program: &std::path::Path,
        arguments: &[String],
        environment: &ClientEnvironment,
    ) -> Result<Self, ClientError> {
        let mut child = Command::new(program)
            .args(arguments)
            .env(env_var::INSTANCE, &environment.instance_url)
            .env(env_var::STATE_DIR, &environment.state_dir)
            .env(env_var::RUNTIME_DIR, &environment.runtime_dir)
            .env(env_var::TOKEN, &environment.token)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(ClientError::Io)?;

        let stdin = child.stdin.take().ok_or(ClientError::Gone)?;
        let stdout = child.stdout.take().ok_or(ClientError::Gone)?;

        // A reader thread rather than a blocking read, so a client that never
        // answers costs the harness a timeout instead of the whole suite.
        let (sender, replies) = mpsc::channel();
        std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                let message = match line {
                    Ok(text) => Ok(text),
                    Err(error) => Err(error.to_string()),
                };
                if sender.send(message).is_err() {
                    return;
                }
            }
        });

        Ok(Self {
            child,
            stdin: Some(stdin),
            replies,
            dead: false,
        })
    }
}

impl ClientProcess for ChildClient {
    fn act(&mut self, action: &Action) -> Result<Reply, ClientError> {
        if self.dead {
            return Err(ClientError::Gone);
        }
        let stdin = self.stdin.as_mut().ok_or(ClientError::Gone)?;
        let mut line = serde_json::to_string(action)
            .map_err(|error| ClientError::Unreadable(error.to_string()))?;
        line.push('\n');
        stdin.write_all(line.as_bytes()).map_err(ClientError::Io)?;
        stdin.flush().map_err(ClientError::Io)?;

        match self.replies.recv_timeout(PATIENCE) {
            Ok(Ok(text)) => serde_json::from_str(&text)
                .map_err(|error| ClientError::Unreadable(format!("{error}: {text}"))),
            Ok(Err(error)) => Err(ClientError::Unreadable(error)),
            Err(RecvTimeoutError::Timeout) => Err(ClientError::Silent),
            Err(RecvTimeoutError::Disconnected) => Err(ClientError::Gone),
        }
    }

    fn kill(&mut self) {
        if self.dead {
            return;
        }
        self.dead = true;
        // Drop stdin first so the child cannot mistake the kill for an orderly
        // end of input and flush on the way out.
        self.stdin = None;
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

impl Drop for ChildClient {
    fn drop(&mut self) {
        self.kill();
    }
}
