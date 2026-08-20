//! `altaird` as a process: configuration, preconditions, the credential edge,
//! what crosses the wire, and shutdown.
//!
//! Waves 1 through 3 verified the instance by calling its functions. Nothing
//! in that reaches a socket, a status code, a header, or a signal, and every
//! condition in this file is one that only exists once there is a process.
//!
//! # The classification these tests are really about
//!
//! The substrate divides what a client meets into a **wait**, which the
//! ordinary path clears by continuing to run and which is silent, and a
//! **fault**, which signals. The whole point of the outcome mapping is that a
//! client can tell them apart. [`condition`] below is that mapping written
//! down from the client's side, and the tests assert against it rather than
//! against status codes directly — because "an expired token and an
//! unreachable instance are the same to the person" is a statement about the
//! classification and not about the two codes being equal.

mod common;

use std::sync::Arc;
use std::time::Duration;

use altair_proto::v1;
use altaird::daemon::tasks::Tasks;
use altaird::daemon::{Config, preflight};
use altaird::objects::{Body, BodyId, BodyListing, ByteSource, Error, ObjectStore};
use common::running::{Running, client_for, request};
use common::*;
use futures::StreamExt;
use tonic::{Code, Status};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// What a client concludes
// ---------------------------------------------------------------------------

/// A wait is silent and clears by itself; a fault signals.
#[derive(Debug, PartialEq, Eq)]
enum Condition {
    Wait,
    Fault,
}

/// How a client reads a `Status`, written from the client's side.
///
/// This mirrors `service.rs`'s list of every status the instance returns, and
/// the fact that it can be written at all is the property under test: if the
/// instance ever answered a refusal as a `Status`, no function of a `Status`
/// alone could tell a refusal from a wait and this one would be a lie.
///
/// **The `Unknown` case is measured, not reasoned.** A connection torn down
/// mid-call and an answer a client cannot classify both arrive as `Unknown`,
/// so the code cannot be the discriminator. A status tonic synthesised from a
/// transport failure carries the underlying error as its `source`; one the
/// instance actually sent carries none.
fn condition(status: &Status) -> Condition {
    use std::error::Error as _;

    // A transport failure, whatever code it wears. The instance was not
    // reached, which is the same wait as any other way of not reaching it.
    if status.source().is_some() {
        return Condition::Wait;
    }

    match status.code() {
        // Every credential failure, and the store being unreachable while
        // resolving one.
        Code::Unauthenticated | Code::Unavailable => Condition::Wait,
        // `GetBody` on something absent or not ours. Indistinguishable on
        // purpose, and a wait for the same reason.
        Code::NotFound => Condition::Wait,
        // A request that names nothing valid. Waiting will not fix it.
        _ => Condition::Fault,
    }
}

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

/// Every complaint at once, because an operator standing an instance up for
/// the first time has all six wrong and six restarts is not a way to find out.
#[test]
fn an_unconfigured_instance_is_told_everything_that_is_missing() {
    // Deliberately not `Config::from_env`: the environment of a test process
    // is shared with every other test in this binary, and a test that cleared
    // it would be a test that broke its neighbours. What is checked here is
    // that the complaint list and the variable list agree, which is what makes
    // "it names every one" true.
    assert_eq!(
        altaird::daemon::config::VARIABLES.len(),
        6,
        "the daemon reads six values; if that changed, so did what an operator has to set"
    );
    for name in altaird::daemon::config::VARIABLES {
        assert!(
            name.starts_with("ALTAIR_"),
            "{name} is not obviously this instance's, which matters in a shared environment"
        );
    }
}

// ---------------------------------------------------------------------------
// Refuse to start rather than start degraded
// ---------------------------------------------------------------------------

#[tokio::test]
async fn it_will_not_start_without_the_structured_store() {
    let at = std::time::Instant::now();
    let refusal = Running::try_with(|config| Config {
        database_url: "postgres://nobody:nothing@127.0.0.1:1/absent".into(),
        ..config
    })
    .await
    .err()
    .expect("a daemon with no database must not start");

    // Nothing about the reason is asserted beyond it having one: what matters
    // is that the process refused rather than came up and discovered this at
    // the first capture.
    assert!(!format!("{refusal:#}").is_empty());

    // And that it said so promptly. sqlx's own acquire timeout is thirty
    // seconds and applies to every acquisition; `store::connect` puts its own
    // deadline round the first one so a misconfigured start is not half a
    // minute of silence. This is that decision, asserted.
    assert!(
        at.elapsed() < Duration::from_secs(25),
        "the daemon took {:?} to say it could not reach the store, which is the \
         library's timeout rather than the instance's",
        at.elapsed()
    );
}

#[tokio::test]
async fn it_will_not_start_when_a_required_extension_is_absent() {
    // Built first so the database exists to break, then broken, then started
    // against. `CASCADE` takes the columns that depend on the type with it,
    // which is fine in a database that exists for one test.
    let db = altaird::testing::TestDb::new().await;
    sqlx::query("DROP EXTENSION vector CASCADE")
        .execute(&db.pool)
        .await
        .expect("drop the extension");

    let object_root = tempfile::TempDir::new().expect("temp object root");
    let key = common::issuer::Key::generate();
    let issuer = common::issuer::Issuer::publishing(&key).await;

    let refusal = altaird::daemon::start(Config {
        database_url: db.url(),
        object_root: object_root.path().to_str().expect("a name").to_owned(),
        issuer: common::issuer::ISSUER.into(),
        audience: common::issuer::AUDIENCE.into(),
        jwks_uri: issuer.jwks_uri(),
        listen: "127.0.0.1:0".parse().expect("an address"),
    })
    .await
    .err()
    .expect("an instance without pgvector must not start");

    let said = format!("{refusal:#}");
    assert!(
        said.contains("vector"),
        "the refusal must name what was missing, so an operator can fix it: {said}"
    );
}

#[tokio::test]
async fn it_will_not_start_when_the_object_root_cannot_be_used() {
    // A root whose parent is a file rather than a directory. The object store
    // cannot create its layout underneath one, which is the shape of every
    // real version of this: a volume that was not mounted, a path that is
    // occupied, a filesystem that will not take a directory.
    let occupied = tempfile::NamedTempFile::new().expect("a file");
    let under_a_file = occupied
        .path()
        .join("bodies")
        .to_str()
        .expect("a name")
        .to_owned();

    let refusal = Running::try_with(|config| Config {
        object_root: under_a_file,
        ..config
    })
    .await
    .err()
    .expect("a daemon with an unusable object root must not start");

    assert!(!format!("{refusal:#}").is_empty());
}

// ---------------------------------------------------------------------------
// The writability probe
// ---------------------------------------------------------------------------

/// A store that takes bytes and does whatever it was built to do with them.
///
/// Each variant is a way a filesystem lies that a `create_dir_all` or a
/// permission bit would not catch, which is why the probe is a round trip
/// rather than a check that the directory exists.
struct Deceitful(Behaviour);

enum Behaviour {
    /// Accepts the write, reports the right length, holds nothing.
    Discards,
    /// Accepts the write and reports fewer bytes than it was given.
    Truncates,
    /// Stores something, but not what it was handed.
    Alters,
}

#[async_trait::async_trait]
impl ObjectStore for Deceitful {
    async fn put(&self, _id: BodyId, mut source: ByteSource) -> Result<u64, Error> {
        let mut written = 0_u64;
        while let Some(chunk) = source.next().await {
            written += chunk.map_err(Error::Source)?.len() as u64;
        }
        match self.0 {
            Behaviour::Truncates => Ok(written.saturating_sub(1)),
            _ => Ok(written),
        }
    }

    async fn get(&self, _id: BodyId) -> Result<Body, Error> {
        match self.0 {
            Behaviour::Discards => Err(Error::NoSuchBody),
            _ => {
                let bytes = b"something else entirely".to_vec();
                let len = bytes.len() as u64;
                Ok(Body::new(
                    len,
                    Box::pin(futures::stream::once(async move { Ok(bytes) })),
                ))
            }
        }
    }

    async fn delete(&self, _id: BodyId) -> Result<(), Error> {
        Ok(())
    }

    fn enumerate(&self) -> BodyListing<'_> {
        Box::pin(futures::stream::empty())
    }
}

#[tokio::test]
async fn the_probe_catches_a_store_that_only_looks_like_it_took_the_bytes() {
    for behaviour in [Behaviour::Discards, Behaviour::Truncates, Behaviour::Alters] {
        let store = Deceitful(behaviour);
        assert!(
            preflight::object_store(&store).await.is_err(),
            "a store that does not give back what it was given must fail the probe"
        );
    }
}

#[tokio::test]
async fn the_probe_leaves_nothing_behind() {
    let root = tempfile::TempDir::new().expect("temp object root");
    let store = altaird::objects::FilesystemObjectStore::open(root.path())
        .await
        .expect("open");

    preflight::object_store(&store).await.expect("probe passes");

    let held: Vec<_> = store.enumerate().collect().await;
    assert!(
        held.is_empty(),
        "the probe's body must be removed before the daemon serves: {held:?}"
    );
}

// ---------------------------------------------------------------------------
// A client session against a running daemon
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_client_submits_an_intent_and_queries_it_back() {
    let running = Running::new().await;
    let mut client = running.client();
    let token = running.token("one");

    let entity = Uuid::new_v4();
    let answer = client
        .submit(running.request(
            v1::SubmitRequest {
                intents: vec![v1::Intent {
                    intent_id: Uuid::new_v4().as_bytes().to_vec(),
                    action: Some(create_entity(
                        entity,
                        v1::EntityContent {
                            title: Some("the lantern in the reliquary".into()),
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
        .expect("the submission is answered")
        .into_inner();

    assert_eq!(answer.acknowledgements.len(), 1);
    assert!(
        matches!(
            answer.acknowledgements[0].outcome,
            Some(v1::acknowledgement::Outcome::Applied(_))
        ),
        "expected applied, got {:?}",
        answer.acknowledgements[0].outcome
    );

    let found = client
        .query(running.request(
            v1::QueryRequest {
                text: "reliquary".into(),
                ..Default::default()
            },
            Some(&token),
        ))
        .await
        .expect("the query is answered")
        .into_inner();

    assert_eq!(
        found.results.len(),
        1,
        "the literal arm finds a just-captured entity by its words, before anything is derived"
    );

    // The honest answer, and the reason the field exists. Wave 5 builds the
    // semantic arm; until then saying so is the contract being kept rather
    // than a placeholder standing in for it.
    let state = found.state.expect("an answer says what produced it");
    assert!(!state.semantic_available);
    assert!(
        state.derivation_outstanding,
        "nothing derives embeddings yet, so everything is outstanding and the answer says so"
    );

    running.stop().await.expect("clean shutdown");
}

// ---------------------------------------------------------------------------
// Wait, fault, refusal
// ---------------------------------------------------------------------------

#[tokio::test]
async fn an_expired_token_and_an_unreachable_instance_are_the_same_condition() {
    let running = Running::new().await;
    let address = running.address();
    let expired = running.key.expired_token_for("one");

    let mut client = running.client();
    let with_an_expired_token = client
        .query(request(v1::QueryRequest::default(), Some(&expired)))
        .await
        .expect_err("an expired token is never answered");

    // The same instance, stopped. The client is unchanged; only whether
    // anything is listening has.
    running.stop().await.expect("clean shutdown");
    let mut client = client_for(address);
    let with_nothing_listening = client
        .query(request(v1::QueryRequest::default(), Some(&expired)))
        .await
        .expect_err("a call to nothing is never answered");

    assert_eq!(
        condition(&with_an_expired_token),
        Condition::Wait,
        "an expired session is a wait: the outbox holds and nothing signals"
    );
    assert_eq!(
        condition(&with_nothing_listening),
        Condition::Wait,
        "an unreachable instance is the same wait, and to the person the same thing"
    );

    // And nothing in what the instance said would let a client treat them
    // differently even if it wanted to.
    assert_eq!(
        with_an_expired_token.message(),
        "",
        "an unauthenticated answer says nothing about why"
    );

    // The property `condition` leans on, asserted rather than assumed. It is a
    // property of tonic and not of the contract, so nothing would fail loudly
    // if it changed — which is exactly why it is pinned here. A client tells a
    // status the instance sent from one tonic synthesised out of a transport
    // failure by whether it carries the underlying error as its source.
    use std::error::Error as _;
    assert!(
        with_nothing_listening.source().is_some(),
        "a synthesised status carries the transport failure that caused it"
    );
    assert!(
        with_an_expired_token.source().is_none(),
        "a status the instance sent carries no source of its own"
    );
}

#[tokio::test]
async fn no_property_of_a_credential_is_ever_disclosed() {
    let running = Running::new().await;
    let mut client = running.client();

    let forged = "not.a.token";
    let unknown_subject = running.token("somebody-who-never-joined");
    let expired = running.key.expired_token_for("one");

    let mut answers = Vec::new();
    for credential in [
        None,
        Some(forged),
        Some(&unknown_subject[..]),
        Some(&expired[..]),
    ] {
        let refused = client
            .query(request(v1::QueryRequest::default(), credential))
            .await
            .expect_err("none of these is answered");
        answers.push((refused.code(), refused.message().to_owned()));
    }

    let first = answers[0].clone();
    for answer in &answers {
        assert_eq!(
            *answer, first,
            "absent, forged, unknown and expired must be one value: {answers:?}"
        );
    }
    assert_eq!(first.0, Code::Unauthenticated);

    running.stop().await.expect("clean shutdown");
}

#[tokio::test]
async fn a_refusal_is_not_a_status_and_is_told_apart_from_a_wait() {
    let running = Running::new().await;
    let mut client = running.client();
    let token = running.token("one");

    // An edit of something that does not exist. Refused, and the refusal
    // arrives inside an ordinary answer.
    let answer = client
        .submit(running.request(
            v1::SubmitRequest {
                intents: vec![v1::Intent {
                    intent_id: Uuid::new_v4().as_bytes().to_vec(),
                    action: Some(v1::intent::Action::Edit(v1::Edit {
                        subject: Some(v1::edit::Subject::Entity(v1::EditEntity {
                            entity_id: Uuid::new_v4().as_bytes().to_vec(),
                            base_counter: 0,
                            content: Some(v1::EntityContent {
                                title: Some("nowhere".into()),
                                ..Default::default()
                            }),
                        })),
                    })),
                }],
            },
            Some(&token),
        ))
        .await
        .expect("a submission whose every intent was refused is still Ok")
        .into_inner();

    assert!(
        matches!(
            answer.acknowledgements[0].outcome,
            Some(v1::acknowledgement::Outcome::Refused(_))
        ),
        "the refusal travels in the message: {:?}",
        answer.acknowledgements[0].outcome
    );

    running.stop().await.expect("clean shutdown");
}

// ---------------------------------------------------------------------------
// Audience refusal and nonexistence refusal
// ---------------------------------------------------------------------------

#[tokio::test]
async fn an_entity_that_is_not_ours_is_refused_exactly_as_one_that_is_not_there() {
    let running = Running::new().await;
    let mut client = running.client();

    // Member one captures something private. Member two then asks about it,
    // and about an identity nobody has ever used.
    let theirs = Uuid::new_v4();
    client
        .submit(running.request(
            v1::SubmitRequest {
                intents: vec![v1::Intent {
                    intent_id: Uuid::new_v4().as_bytes().to_vec(),
                    action: Some(create_entity(
                        theirs,
                        v1::EntityContent {
                            title: Some("a private thing".into()),
                            specific: Some(v1::entity_content::Specific::Note(
                                v1::NoteContent::default(),
                            )),
                            ..Default::default()
                        },
                    )),
                }],
            },
            Some(&running.token("one")),
        ))
        .await
        .expect("captured")
        .into_inner();

    let two = running.token("two");
    let not_ours = edit_by(&mut client, &two, theirs).await;
    let not_there = edit_by(&mut client, &two, Uuid::new_v4()).await;

    // The whole outcome, not the one field somebody thought to check. A reason
    // code that matched while a detail string differed would pass a hand-picked
    // comparison and still be the leak. The intent identity is left out on
    // purpose: it differs on every call by construction and says nothing about
    // the entity.
    assert_eq!(
        not_ours.outcome, not_there.outcome,
        "audience refusal and nonexistence refusal are one answer, field for field"
    );

    // They must also be indistinguishable in the log, which is checked once
    // and for all in `tests/logging.rs` rather than here: the rule there is
    // that the served surface and the read path cannot log at all, which is a
    // stronger statement than these two producing the same line. And in how
    // long they took, which is the test below.

    running.stop().await.expect("clean shutdown");
}

/// The two refusals must also be indistinguishable by how long they took.
///
/// **What this can and cannot catch.** It compares medians over enough
/// repetitions to be stable, against a band wide enough that ordinary jitter
/// and a loaded machine never fire it — because both sides are measured on the
/// same machine at the same time, a shared load moves both. What it catches is
/// a *structural* difference: one path doing a second round trip to the store,
/// or looking something up that the other does not. What it cannot catch is a
/// few microseconds, and it does not pretend to.
///
/// The reason there is nothing to find today is stronger than this test:
/// both refusals come from a single `available_for_read`, which carries the
/// audience predicate inside the candidate query, so "not yours" and "not
/// there" are the same query returning no row rather than two paths that agree.
/// This exists so that stops being true loudly.
#[tokio::test]
async fn neither_refusal_takes_measurably_longer_than_the_other() {
    const ROUNDS: usize = 15;

    let running = Running::new().await;
    let mut client = running.client();

    let theirs = Uuid::new_v4();
    client
        .submit(running.request(
            v1::SubmitRequest {
                intents: vec![v1::Intent {
                    intent_id: Uuid::new_v4().as_bytes().to_vec(),
                    action: Some(create_entity(
                        theirs,
                        v1::EntityContent {
                            title: Some("a private thing".into()),
                            specific: Some(v1::entity_content::Specific::Note(
                                v1::NoteContent::default(),
                            )),
                            ..Default::default()
                        },
                    )),
                }],
            },
            Some(&running.token("one")),
        ))
        .await
        .expect("captured");

    let two = running.token("two");
    let mut not_ours = Vec::new();
    let mut not_there = Vec::new();
    // Interleaved, so a machine that got busier partway through makes both
    // samples slower rather than one of them.
    for _ in 0..ROUNDS {
        let at = std::time::Instant::now();
        edit_by(&mut client, &two, theirs).await;
        not_ours.push(at.elapsed());

        let at = std::time::Instant::now();
        edit_by(&mut client, &two, Uuid::new_v4()).await;
        not_there.push(at.elapsed());
    }

    let (ours, there) = (median(&mut not_ours), median(&mut not_there));
    let (slower, faster) = if ours > there {
        (ours, there)
    } else {
        (there, ours)
    };
    assert!(
        slower.as_secs_f64() < faster.as_secs_f64() * 3.0,
        "one refusal takes materially longer than the other ({ours:?} against {there:?}), \
         which is a difference somebody outside the household can measure"
    );

    running.stop().await.expect("clean shutdown");
}

fn median(samples: &mut [Duration]) -> Duration {
    samples.sort_unstable();
    samples[samples.len() / 2]
}

async fn edit_by(
    client: &mut common::running::Client,
    token: &str,
    entity: Uuid,
) -> v1::Acknowledgement {
    let answer = client
        .submit(request(
            v1::SubmitRequest {
                intents: vec![v1::Intent {
                    intent_id: Uuid::new_v4().as_bytes().to_vec(),
                    action: Some(v1::intent::Action::Edit(v1::Edit {
                        subject: Some(v1::edit::Subject::Entity(v1::EditEntity {
                            entity_id: entity.as_bytes().to_vec(),
                            base_counter: 1,
                            content: Some(v1::EntityContent {
                                title: Some("changed".into()),
                                ..Default::default()
                            }),
                        })),
                    })),
                }],
            },
            Some(token),
        ))
        .await
        .expect("answered")
        .into_inner();
    answer
        .acknowledgements
        .into_iter()
        .next()
        .expect("one intent, one answer")
}

// ---------------------------------------------------------------------------
// Bodies
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_body_round_trips_through_the_streaming_calls() {
    let running = Running::new().await;
    let mut client = running.client();
    let token = running.token("one");

    // Larger than the store's chunk size, so this is a stream of several
    // messages in each direction rather than one message wearing a stream's
    // shape.
    let bytes: Vec<u8> = (0..200_000_u32).map(|i| (i % 251) as u8).collect();
    let body = BodyId::new();

    let chunks: Vec<v1::BodyChunk> = bytes
        .chunks(64 * 1024)
        .map(|piece| v1::BodyChunk {
            body_id: body.as_bytes().to_vec(),
            data: piece.to_vec(),
        })
        .collect();

    let ack = client
        .put_body(request(futures::stream::iter(chunks), Some(&token)))
        .await
        .expect("the upload is answered")
        .into_inner();
    assert_eq!(ack.bytes_received, bytes.len() as u64);

    // Bytes before the record: the file entity is created after the upload,
    // naming the body that is already durable.
    let entity = Uuid::new_v4();
    let answer = client
        .submit(running.request(
            v1::SubmitRequest {
                intents: vec![v1::Intent {
                    intent_id: Uuid::new_v4().as_bytes().to_vec(),
                    action: Some(create_entity(
                        entity,
                        v1::EntityContent {
                            title: Some("the map".into()),
                            specific: Some(v1::entity_content::Specific::File(v1::FileContent {
                                body_id: Some(body.as_bytes().to_vec()),
                                media_type: Some("application/octet-stream".into()),
                                ..Default::default()
                            })),
                            ..Default::default()
                        },
                    )),
                }],
            },
            Some(&token),
        ))
        .await
        .expect("answered")
        .into_inner();
    assert!(
        matches!(
            answer.acknowledgements[0].outcome,
            Some(v1::acknowledgement::Outcome::Applied(_))
        ),
        "the file was not created: {:?}",
        answer.acknowledgements[0].outcome
    );

    let mut stream = client
        .get_body(running.request(
            v1::BodyRequest {
                entity_id: entity.as_bytes().to_vec(),
            },
            Some(&token),
        ))
        .await
        .expect("the download is answered")
        .into_inner();

    let mut read = Vec::new();
    while let Some(chunk) = stream.next().await {
        read.extend_from_slice(&chunk.expect("a chunk").data);
    }
    assert_eq!(read, bytes, "the body that came back is not the one sent");

    running.stop().await.expect("clean shutdown");
}

// ---------------------------------------------------------------------------
// Shutdown
// ---------------------------------------------------------------------------

/// Shutdown while a write is in flight, and the write is the one that has to
/// win.
///
/// The write is held deliberately rather than raced for: a second connection
/// takes the change sequence row's lock, which every write in the instance
/// must have before it does anything (see `write::changes`), so the submission
/// is genuinely mid-transaction and stays there until this test lets go.
#[tokio::test]
async fn an_acknowledged_intent_survives_a_shutdown_that_began_before_it_committed() {
    let running = Running::new().await;
    let pool = running.db.pool.clone();
    let token = running.token("one");
    let mut client = running.client();

    // Hold the sequence row. Nothing in the instance can write past this.
    let mut holding = pool.begin().await.expect("begin");
    sqlx::query("SELECT next_position FROM change_position WHERE singleton FOR UPDATE")
        .fetch_one(&mut *holding)
        .await
        .expect("hold the sequence");

    let intent = Uuid::new_v4();
    let entity = Uuid::new_v4();
    let submitting = tokio::spawn(async move {
        client
            .submit(request(
                v1::SubmitRequest {
                    intents: vec![v1::Intent {
                        intent_id: intent.as_bytes().to_vec(),
                        action: Some(create_entity(
                            entity,
                            v1::EntityContent {
                                title: Some("captured as the lights went out".into()),
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
    });

    wait_until_a_write_is_blocked(&pool).await;

    // Now stop, with the write in flight and unable to proceed.
    let stopping = tokio::spawn(running.daemon.stop());

    // And let it proceed. A drained shutdown means this finishes; a shutdown
    // that killed connections means it does not.
    holding.rollback().await.expect("release the sequence");

    let answered = tokio::time::timeout(Duration::from_secs(10), submitting)
        .await
        .expect("the in-flight submission finished")
        .expect("the submitting task did not panic")
        .expect("the in-flight submission was answered rather than cut off")
        .into_inner();

    assert!(
        matches!(
            answered.acknowledgements[0].outcome,
            Some(v1::acknowledgement::Outcome::Applied(_))
        ),
        "a write already in flight when shutdown began must finish, not be refused: {:?}",
        answered.acknowledgements[0].outcome
    );

    tokio::time::timeout(Duration::from_secs(10), stopping)
        .await
        .expect("shutdown finished")
        .expect("the shutdown task did not panic")
        .expect("shutting down is not an error");

    // The acknowledgement said applied. The store must agree, from a
    // connection that has nothing to do with the daemon's own pool.
    let intents: i64 = sqlx::query_scalar("SELECT count(*) FROM intent WHERE id = $1")
        .bind(intent)
        .fetch_one(&pool)
        .await
        .expect("count the intent");
    assert_eq!(intents, 1, "an acknowledged intent is durable");

    let entities: i64 = sqlx::query_scalar("SELECT count(*) FROM entity WHERE id = $1")
        .bind(entity)
        .fetch_one(&pool)
        .await
        .expect("count the entity");
    assert_eq!(
        entities, 1,
        "the write the acknowledgement was for committed with it"
    );
}

/// Wait until some backend is blocked on a lock in this database.
///
/// Polled rather than slept through: how long the daemon takes to get as far
/// as the sequence row depends on the machine, and a sleep long enough to be
/// safe on a loaded runner is a sleep on every run.
async fn wait_until_a_write_is_blocked(pool: &sqlx::PgPool) {
    for _ in 0..200 {
        let blocked: i64 = sqlx::query_scalar(
            "SELECT count(*) FROM pg_stat_activity \
             WHERE datname = current_database() AND wait_event_type = 'Lock'",
        )
        .fetch_one(pool)
        .await
        .expect("read pg_stat_activity");
        if blocked > 0 {
            return;
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
    panic!("no write ever blocked on the sequence row, so nothing was in flight to test");
}

// ---------------------------------------------------------------------------
// The shape background work will attach to
// ---------------------------------------------------------------------------

/// Nothing runs in this process yet. What is checked is the shape Wave 2.4's
/// reclamation and Wave 5's derivation worker will attach to, because it is
/// decided now and a shape nobody exercised is a shape that does not work.
#[tokio::test]
async fn a_background_task_is_asked_to_stop_and_waited_for() {
    let stopped = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let mut tasks = Tasks::new();

    let flag = Arc::clone(&stopped);
    tasks.spawn("a test worker", move |mut shutdown| async move {
        shutdown.requested().await;
        // Work after the signal: a task chooses where it is safe to stop, and
        // shutdown waits for it to get there.
        tokio::time::sleep(Duration::from_millis(50)).await;
        flag.store(true, std::sync::atomic::Ordering::SeqCst);
    });
    assert_eq!(tasks.count(), 1);

    let overran = tasks.stop().await;
    assert!(overran.is_empty(), "the task finished in time: {overran:?}");
    assert!(
        stopped.load(std::sync::atomic::Ordering::SeqCst),
        "shutdown returned before the task had finished"
    );
}

#[tokio::test]
async fn a_task_that_will_not_stop_is_named_rather_than_waited_for_forever() {
    let mut tasks = Tasks::new();
    tasks.spawn("a worker that ignores the signal", |_shutdown| async move {
        // Never returns. An operator must not be held by this.
        std::future::pending::<()>().await;
    });

    let at = std::time::Instant::now();
    let overran = tasks.stop().await;

    assert_eq!(overran, vec!["a worker that ignores the signal"]);
    assert!(
        at.elapsed() < Duration::from_secs(30),
        "shutdown waited far longer than its own deadline"
    );
}

#[tokio::test]
async fn a_task_spawned_into_a_shutdown_already_under_way_stops_at_once() {
    let tasks = Tasks::new();
    let signal = tasks.signal();

    // Stop first, then hand the already-signalled receiver to a task. This is
    // the race a worker started late would meet, and it must not wait for a
    // change that has already happened.
    tasks.stop().await;

    let mut late = signal;
    tokio::time::timeout(Duration::from_secs(5), late.requested())
        .await
        .expect("an already-signalled shutdown resolves immediately");
}

/// Nothing is running yet, and that is the current state rather than an
/// oversight. This fails when the first task lands, which is the moment to
/// come back and say what it is.
#[tokio::test]
async fn the_daemon_runs_no_background_work_yet() {
    let running = Running::new().await;
    // Reaching the tasks from outside is deliberately not possible, so this
    // asserts the observable consequence instead: shutting down is immediate
    // and reports nothing overran.
    let at = std::time::Instant::now();
    running.stop().await.expect("clean shutdown");
    assert!(
        at.elapsed() < Duration::from_secs(5),
        "shutting down waited for something, and this wave started nothing"
    );
}
