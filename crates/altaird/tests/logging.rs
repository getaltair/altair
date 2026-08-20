//! What the daemon writes down, and what it must never write down.
//!
//! **A whole test binary for one subscriber.** A tracing subscriber is
//! installed globally, once per process, so a file that installs one owns its
//! process. Cargo gives each integration test file its own binary, which is
//! what makes that affordable — and putting this anywhere else would mean
//! either testing a subscriber somebody else configured or fighting over which
//! test got to install one.
//!
//! # The trap this exists for
//!
//! `crate::daemon::logging` states the policy: the daemon logs its own
//! lifecycle and its own faults, and nothing about a request. That policy is
//! broken not by anyone writing a logging call, but by a library default. sqlx
//! logs every statement — at `DEBUG` ordinarily, and **at `WARN`, with the
//! whole SQL attached, the moment one takes longer than a second**. A write
//! waiting on the change sequence row crosses that threshold routinely, so the
//! condition is not exotic; it is the ordinary busy Tuesday. The read path is
//! required to keep no record of what was asked, and this is how that
//! invariant would be lost without anybody deciding to lose it.
//!
//! # Why the slow path is induced on purpose
//!
//! A test that asserted "no SQL appears in the log" over an idle daemon would
//! pass over a condition that never arose, and would go on passing if the
//! guard were removed. So this test **holds the change sequence row** until a
//! write has been blocked for longer than sqlx's slow-statement threshold,
//! which is exactly the emission being guarded against, and only then reads
//! the log.
//!
//! # Why the filter is turned off here
//!
//! There are two defences and only one of them is a guarantee. The default
//! subscriber filter silences sqlx's statement target, and
//! `store::connect` stops the statements being emitted at all. With the
//! default filter in place this test passes either way — it was watched doing
//! exactly that, with the source-side guard deleted — which makes it a check
//! that cannot fail.
//!
//! So it installs with the silencing **removed**, which is both the only way
//! to test the defence that matters and an honest model of an operator turning
//! levels up at two in the morning to chase something unrelated.
//!
//! Run once with `disable_statement_logging` removed from `store::connect` and
//! this fails, printing the statement it found. That is the failure it was
//! written against, and it has been watched.

mod common;

use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use altair_proto::v1;
use common::running::{Running, request};
use common::*;
use uuid::Uuid;

/// sqlx logs a statement as slow once it crosses one second. Comfortably past
/// it, so a fast machine still induces the emission.
const LONGER_THAN_SLOW: Duration = Duration::from_millis(1_500);

/// The daemon's own filter with exactly one thing taken out: the silencing of
/// sqlx's statement target.
///
/// **Precisely that target, and not `sqlx=trace`.** Everything else below this
/// crate stays at `warn`, because the subject is the guarantee that survives
/// the default filter being overridden, not a survey of what libraries say
/// when turned all the way up. Widening it further also finds
/// `sqlx_postgres::connection::resolve`, which prints its own catalog SQL at
/// `TRACE` under a different target — internal, carrying nothing a person
/// asked for, and a reason the default filter holds everything below the
/// instance at `warn` rather than trusting one target's name.
const ADVERSARIAL_FILTER: &str = "warn,altaird=trace,sqlx::query=trace";

/// A term that exists nowhere but in one request.
///
/// Distinctive enough that finding it anywhere in the log is unambiguous — it
/// cannot have come from a module path, a library's message, or a UUID.
const CANARY: &str = "zqxjvwmb-only-in-a-query";

/// Words that appear only inside SQL this instance issues.
///
/// `change_position` is the sequence row's table, which every write touches;
/// `pg_extension` is read once at startup. Neither is a word any library or
/// module path uses, so either of them in the log means a statement reached it.
const ONLY_IN_STATEMENTS: &[&str] = &["change_position", "pg_extension", "SELECT", "INSERT INTO"];

// ---------------------------------------------------------------------------
// A log that can be read back
// ---------------------------------------------------------------------------

#[derive(Clone, Default)]
struct Captured(Arc<Mutex<Vec<u8>>>);

impl Captured {
    fn read(&self) -> String {
        String::from_utf8_lossy(&self.0.lock().expect("the log is not poisoned")).into_owned()
    }

    /// Forget everything so far.
    ///
    /// **Used exactly once, and for a reason worth stating.** This test's own
    /// harness talks to Postgres — it branches a database, seeds a household,
    /// seeds two memberships — over a pool that is not the daemon's and does
    /// not disable statement logging. That SQL is the test's, not the
    /// instance's, and leaving it in the buffer would make the assertions fail
    /// on the scaffolding rather than on the subject. What is checked after
    /// this point is what the daemon produced.
    fn clear(&self) {
        self.0.lock().expect("the log is not poisoned").clear();
    }
}

impl io::Write for Captured {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0
            .lock()
            .expect("the log is not poisoned")
            .extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for Captured {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

// ---------------------------------------------------------------------------
// The one test in this process
// ---------------------------------------------------------------------------

/// Everything about the log, in one test, because there is one subscriber.
///
/// Splitting this into several would mean several processes or a shared
/// mutable buffer with an ordering problem; one test that says what it checked
/// at each step is the honest shape.
#[tokio::test(flavor = "multi_thread")]
async fn the_log_says_nothing_about_what_anybody_asked() {
    let log = Captured::default();
    // Deliberately not the default filter: `sqlx=trace` is exactly what the
    // default silences, so this puts the emission back and leaves only the
    // source-side guard standing. See the module documentation.
    altaird::daemon::logging::install_with_filter(log.clone(), ADVERSARIAL_FILTER)
        .expect("this process installs exactly one subscriber");

    assert!(
        !altaird::daemon::logging::DEFAULT_FILTER.contains("sqlx=trace"),
        "the filter under test must be more permissive than the one the daemon runs with, \
         or this test is easier than reality"
    );

    let running = Running::new().await;
    let token = running.token("one");
    let mut client = running.client();

    // The daemon's own lifecycle, which is what the log is for. Asserted here
    // rather than at the end, because the buffer is about to be cleared.
    assert!(
        log.read().contains("serving"),
        "the daemon's own lifecycle is what the log is for, and it said nothing.\n\n{}",
        log.read()
    );
    log.clear();

    // The test's own connection to the same database, opened the way the
    // daemon opens its own — quiet by construction, so what this test does to
    // Postgres does not end up being read back as what the instance did.
    let pool = altaird::store::connect(&running.db.url())
        .await
        .expect("a quiet connection to the test database");

    // --- a query carrying a term that exists nowhere else -----------------

    client
        .query(request(
            v1::QueryRequest {
                text: CANARY.into(),
                ..Default::default()
            },
            Some(&token),
        ))
        .await
        .expect("the query is answered");

    // --- a write held long enough to be a slow statement -------------------

    let mut holding = pool.begin().await.expect("begin");
    sqlx::query("SELECT next_position FROM change_position WHERE singleton FOR UPDATE")
        .fetch_one(&mut *holding)
        .await
        .expect("hold the sequence");

    let entity = Uuid::new_v4();
    let submitting = tokio::spawn({
        let mut client = running.client();
        let token = token.clone();
        async move {
            client
                .submit(request(
                    v1::SubmitRequest {
                        intents: vec![v1::Intent {
                            intent_id: Uuid::new_v4().as_bytes().to_vec(),
                            action: Some(create_entity(
                                entity,
                                v1::EntityContent {
                                    title: Some(CANARY.into()),
                                    specific: Some(v1::entity_content::Specific::Note(
                                        v1::NoteContent::default(),
                                    )),
                                    ..Default::default()
                                },
                            )),
                        }],
                    },
                    Some(&token),
                ))
                .await
        }
    });

    // Long enough that the blocked statement is well past sqlx's threshold.
    // This is the one deliberate sleep in the suite and it is the subject of
    // the test rather than a way of waiting for something else.
    tokio::time::sleep(LONGER_THAN_SLOW).await;
    holding.rollback().await.expect("release the sequence");

    submitting
        .await
        .expect("the submitting task did not panic")
        .expect("the write finished");

    // --- what the log may contain -----------------------------------------

    let written = log.read();

    assert!(
        !written.contains(CANARY),
        "a term that appeared only in a request reached the log. \
         The read path keeps no record of what was asked.\n\n{written}"
    );

    for statement in ONLY_IN_STATEMENTS {
        assert!(
            !written.contains(statement),
            "{statement:?} reached the log, so a statement did. \
             `store::connect` disables sqlx's statement logging at the source, \
             and a write was deliberately held past the slow-statement threshold \
             here to make sure that is being exercised.\n\n{written}"
        );
    }

    // Nothing that identifies who asked, either. A log line naming a member is
    // a record of who, which is the same disclosure as a record of what.
    //
    // The subject itself is not in this list, and deliberately: the harness
    // spells it `one`, which is too ordinary a word for a substring search to
    // say anything about. The credential that carried it is checked instead,
    // and it contains the subject.
    for identity in [
        running.one.to_string(),
        running.two.to_string(),
        running.household.to_string(),
        entity.to_string(),
        token.clone(),
    ] {
        assert!(
            !written.contains(&identity),
            "{identity:?} reached the log. Who asked is a record too.\n\n{written}"
        );
    }

    running.stop().await.expect("clean shutdown");

    // Also the guard against a vacuous pass: everything above is an assertion
    // that something is *absent*, and a subscriber that had quietly stopped
    // writing would satisfy all of it. Stopping is a lifecycle event, it comes
    // after the clear, and it proves the log was still being written to while
    // the requests it says nothing about were being served.
    let after = log.read();
    assert!(
        after.contains("shutting down"),
        "stopping is a lifecycle event and belongs in the log.\n\n{after}"
    );
}

// ---------------------------------------------------------------------------
// The structural half
// ---------------------------------------------------------------------------

/// The test above catches a line that was written. This catches the call that
/// would write one, before it ever runs.
///
/// The policy is "nothing about a request", and the two places a request lives
/// are the served surface and the read path. Neither has any business emitting
/// anything: a log line there is about somebody's request whatever it says,
/// and the read path in particular is required to keep no record of what was
/// asked. So the rule is not "log carefully here", it is "do not log here",
/// which is a rule a scan can enforce and a reviewer cannot forget.
///
/// The write path is deliberately not covered. An intent that could not be
/// applied because the store failed is the instance describing itself, and
/// there is a real argument for saying so — the same argument does not exist
/// for a query.
#[test]
fn nothing_on_the_request_paths_can_log_at_all() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    let mut silent: Vec<std::path::PathBuf> = Vec::new();
    collect(&src.join("read"), &mut silent);
    silent.push(src.join("service.rs"));
    assert!(
        silent.len() > 1,
        "the read path and the served surface are what this is about, and it found {silent:?}"
    );

    // Assembled, so this file is not an occurrence of itself.
    let macros: Vec<String> = ["info", "warn", "error", "debug", "trace"]
        .iter()
        .map(|level| format!("{level}!"))
        .chain(std::iter::once(["tracing", "::"].concat()))
        .collect();

    let mut offenders = Vec::new();
    for path in silent {
        let source = std::fs::read_to_string(&path).expect("a source file");
        for (number, line) in source.lines().enumerate() {
            // Prose in a doc comment may say the word; a call is code.
            if line.trim_start().starts_with("//") {
                continue;
            }
            for shape in &macros {
                if line.contains(shape.as_str()) {
                    offenders.push(format!(
                        "{}:{}: {}",
                        path.display(),
                        number + 1,
                        line.trim()
                    ));
                }
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "the read path and the served surface log nothing at all — not the text, not the \
         member, not how long it took. See `daemon::logging` for why the rule is blanket \
         rather than careful.\n\n{}",
        offenders.join("\n")
    );
}

fn collect(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}
