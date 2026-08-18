//! The tables v0 defines and deliberately does not write.
//!
//! # Why this exists
//!
//! `altair-v0-scope.md` defers versions, text extraction from files, and
//! consumption and purchase logs. Migration one nevertheless creates
//! `entity_version`, `derived_text`, `embedding`, `derivation_queue` and
//! `event_record`. **That is not a contradiction.** The schema is an adopted
//! artefact and is not re-derived; a table nothing writes costs nothing at
//! runtime and saves a migration later. The scope defers *behaviour*, and no
//! behaviour exists for any of them. On those terms the two documents agree.
//!
//! The gap is narrower, and it is in the code rather than the documents:
//! **nothing marks these tables as deliberately unwritten.** A lane that needs
//! somewhere to put a derived string finds `derived_text` sitting there,
//! correctly shaped, with no signal that reaching it is out of scope — and the
//! failure is silent, because writing to a table nothing reads breaks no test.
//!
//! `DEPENDENT_TABLES` in `write/entity.rs` is the precedent. Forgetting a table
//! there is silent too, and the answer was a named constant plus a test that
//! walks it. This is the same shape pointed the other way: that list says what
//! erasure must remove, this one says what nothing may write.
//!
//! # What this test is not
//!
//! **Not a claim that these tables are wrong to exist**, and not an argument
//! for deleting them. Each has a named wave. When that wave arrives, the entry
//! comes out of [`UNWRITTEN`] with the same one-line edit that put it in, and
//! the reason recorded beside it says which wave that is.

mod common;

use altair_proto::v1;
use common::*;
use sqlx::Row;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// The list
// ---------------------------------------------------------------------------

/// Every table v0 defines and writes nothing into, with the reason.
///
/// The reason travels with the entry so that a failure names what was violated
/// rather than only which table grew a row. Adding a table is one line;
/// removing one when its wave lands is the same line.
const UNWRITTEN: &[(&str, &str)] = &[
    // `altair-v0-scope.md`: "Deferred: versions, text extraction from files."
    (
        "entity_version",
        "version history is deferred in v0; nothing captures a version",
    ),
    // The same sentence, and Wave 5 besides: derived text is produced by the
    // derivation worker, which does not exist. `specific/knowledge.rs` refuses
    // a person's correction to extracted text for exactly this reason.
    (
        "derived_text",
        "text extraction is deferred in v0 and derivation arrives at Wave 5",
    ),
    // Wave 5 chooses the embedding model, and the schema's dimension is a
    // placeholder until it does — the one value in migration one that is not
    // decided. Writing a vector against a placeholder dimension would have to
    // be re-embedded.
    (
        "embedding",
        "the embedding model is chosen at Wave 5 and the schema's dimension is \
         still a placeholder",
    ),
    // The queue is an optimisation over work computable from provenance, and
    // there is no worker to claim from it. Outstanding derivation is computed
    // from the store, never from the queue, so an empty queue costs nothing.
    (
        "derivation_queue",
        "the derivation worker arrives at Wave 5; outstanding work is computed \
         from provenance, never from the queue",
    ),
    // `altair-v0-scope.md`: Tracking's "Deferred: consumption and purchase
    // logs, low-stock thresholds, expiry, barcode capture, shopping lists." An
    // event record is that log. The schema calls them immutable facts,
    // appended and corrected by appending; v0 appends none.
    (
        "event_record",
        "consumption and purchase logs are deferred in v0",
    ),
];

/// Tables that look like candidates and are deliberately **not** listed.
///
/// Recorded here rather than left to be re-argued, in the register the design
/// documents use for a rejected alternative. Each would make this test wrong
/// in a different way.
///
/// - **`block`** — the write path writes it, now that the Bodies lane has
///   landed: `write/body.rs` inserts and deletes blocks as a body divides.
///   [`exercise`] writes one deliberately and
///   [`the_exercise_actually_writes_something`] checks it landed, so this
///   exclusion is demonstrated rather than asserted.
/// - **`template` and `template_property`** — in force, not deferred. Migration
///   one's gap (i) records the scope's ambiguity and resolves it on the reading
///   that templates are in v0, and `specific/tracking.rs` serves `template_id`
///   on both types and reads `template` to validate it. A template is declared
///   out of band because no wire message declares one yet, exactly as a
///   relation type is; forbidding the row would forbid the feature.
/// - **`instance_config`** — the retention constants are Wave 2.4's named
///   trigger, and the schema says v0 runs on the values set with the build.
///   A row here is the expected state rather than a violation.
/// - **`household`, `membership`, `relation_type`** — reference and bootstrap
///   data, populated out of band and non-empty in every test in this suite.
///   The write path does not create them and was never meant to.
const NOT_LISTED: &[&str] = &[
    "block",
    "template",
    "template_property",
    "instance_config",
    "household",
    "membership",
    "relation_type",
];

async fn rows_in(world: &World, table: &str) -> i64 {
    let sql = format!("SELECT count(*) AS n FROM {table}");
    sqlx::query(sqlx::AssertSqlSafe(sql))
        .fetch_one(&world.db.pool)
        .await
        .unwrap_or_else(|e| panic!("counting {table}: {e}"))
        .try_get("n")
        .expect("n")
}

// ---------------------------------------------------------------------------
// Exercising the write path
// ---------------------------------------------------------------------------

/// Put the write path through its range, so that "still empty" means something.
///
/// **Reused rather than novel**, which is where the value is: this asserts
/// against the ordinary write path rather than a setup invented to be safe.
/// Every served entity type is created, content is edited, a relation is formed
/// and removed, and an entity is removed, restored, and erased — the erase in
/// particular, because that is the one path that names these tables at all,
/// in `DEPENDENT_TABLES`.
///
/// **A body is written on purpose**, now that the Bodies lane has landed. That
/// path divides text into blocks and writes them, and it is the nearest thing
/// in the write path to producing derived content — which makes it precisely
/// the case worth pointing at `derived_text`. Derived text is the derivation
/// worker's at Wave 5; dividing a body is not deriving anything.
///
/// A file is not created: that is refused until Wave 2.3, which is another
/// lane's refusal to expire and not what this test is about.
///
/// Every submission is checked for having applied. An exercise whose writes
/// were quietly refused would leave every table empty and pass, which is the
/// one way this file could lie.
async fn exercise(world: &World) {
    use v1::entity_content::Specific;

    // One of every type the create path serves, each addressing nothing, which
    // is the state a bare create leaves them in.
    let bare = [
        Specific::Campaign(v1::CampaignContent::default()),
        Specific::Arc(v1::ArcContent::default()),
        Specific::Quest(v1::QuestContent::default()),
        Specific::Note(v1::NoteContent::default()),
        Specific::Item(v1::ItemContent::default()),
        Specific::Location(v1::LocationContent::default()),
        Specific::ShoppingList(v1::ShoppingListContent::default()),
        Specific::Category(v1::CategoryContent::default()),
    ];
    for specific in bare {
        world
            .create(&world.one, specific, v1::EntityContent::default())
            .await;
    }

    // Type content that actually moves, on the two types this lane and the
    // spine settled. A part that reaches a side table is the interesting case:
    // it is the write furthest from the shared model.
    let holder = world
        .create(
            &world.one,
            Specific::Category(v1::CategoryContent {
                creation_default_audience_member_ids: vec![
                    world.two.membership_id().as_bytes().to_vec(),
                ],
                ..Default::default()
            }),
            v1::EntityContent::default(),
        )
        .await;

    let campaign = world
        .create(
            &world.one,
            Specific::Campaign(v1::CampaignContent {
                state: Some(v1::GuidanceState::Working as i32),
                cleared: Vec::new(),
            }),
            v1::EntityContent {
                title: Some("the kitchen".into()),
                category_id: Some(holder.as_uuid().as_bytes().to_vec()),
                ..Default::default()
            },
        )
        .await;

    // The shared model: a title, a labelled date, an assignment, an audience.
    let base = world.counter(campaign).await as u64;
    applied_or_panic(
        world
            .submit(
                &world.one,
                edit_entity(
                    campaign,
                    base,
                    v1::EntityContent {
                        title: Some("the kitchen, renamed".into()),
                        dates: vec![v1::LabelledDate {
                            label: "due".into(),
                            when: Some(timestamp(chrono::Utc::now())),
                            bring_forward: false,
                        }],
                        assigned_member_ids: vec![world.two.membership_id().as_bytes().to_vec()],
                        audience_member_ids: vec![world.two.membership_id().as_bytes().to_vec()],
                        bulk: Some(true),
                        ..Default::default()
                    },
                ),
            )
            .await,
        "editing the shared model",
    );

    // A body, which divides into blocks and writes them. Several paragraphs, so
    // the division has something to do and the result is more than one row.
    let noted = world
        .create(
            &world.one,
            Specific::Note(v1::NoteContent {
                body: Some(
                    "The kettle is descaled twice a year.\n\n\
                     Vinegar, an hour, then two rinses.\n\n\
                     The filter is a separate job and is not this one."
                        .into(),
                ),
                cleared: Vec::new(),
            }),
            v1::EntityContent {
                title: Some("descaling".into()),
                ..Default::default()
            },
        )
        .await;

    // And the body edited, so blocks are matched against what is held and only
    // what changed is rewritten — the path that deletes as well as inserts.
    let base = world.counter(noted).await as u64;
    applied_or_panic(
        world
            .submit(
                &world.one,
                edit_entity(
                    noted,
                    base,
                    v1::EntityContent {
                        specific: Some(Specific::Note(v1::NoteContent {
                            body: Some(
                                "The kettle is descaled twice a year.\n\n\
                                 Vinegar, an hour, then three rinses."
                                    .into(),
                            ),
                            cleared: Vec::new(),
                        })),
                        ..Default::default()
                    },
                ),
            )
            .await,
        "editing a body",
    );

    // A relation, formed and then removed.
    let other = world.note(&world.one, "something to point at").await;
    let relation = Uuid::new_v4();
    applied_or_panic(
        world
            .submit(
                &world.one,
                create_relation(relation, relation_between(campaign, other)),
            )
            .await,
        "forming a relation",
    );
    applied_or_panic(
        world.submit(&world.one, remove(&[], &[relation])).await,
        "removing a relation",
    );

    // Removal, restoration, and erasure — the last being the only path that
    // names the unwritten tables at all.
    let doomed = world.note(&world.one, "removed, restored, erased").await;
    applied_or_panic(
        world.submit(&world.one, remove(&[doomed], &[])).await,
        "removing an entity",
    );
    applied_or_panic(
        world.submit(&world.one, restore(doomed, false)).await,
        "restoring an entity",
    );
    applied_or_panic(
        world.submit(&world.one, erase(&[doomed])).await,
        "erasing an entity",
    );
}

/// Every write in [`exercise`] has to have landed.
///
/// Without this the file's central assertion is vacuous in the worst way: a
/// change that made the whole write path refuse would leave every table empty
/// and turn this suite green. `World::create` already asserts as much for a
/// create; this is the same check for everything submitted directly.
fn applied_or_panic(ack: v1::Acknowledgement, what: &str) {
    assert!(
        matches!(ack.outcome, Some(v1::acknowledgement::Outcome::Applied(_))),
        "{what} did not apply, so the write path was never exercised: {:?}",
        ack.outcome
    );
}

// ---------------------------------------------------------------------------
// The assertions
// ---------------------------------------------------------------------------

/// **The point of the file.** Exercise the write path, then find every
/// deliberately-unwritten table exactly as empty as migration one left it.
#[tokio::test]
async fn the_write_path_writes_none_of_the_tables_v0_defers() {
    let world = World::new().await;
    exercise(&world).await;

    for (table, why) in UNWRITTEN {
        let n = rows_in(&world, table).await;
        assert_eq!(
            n, 0,
            "the write path put {n} row(s) in `{table}`, which v0 does not \
             write: {why}. If the wave that owns it has landed, remove the \
             entry from UNWRITTEN in this file — do not loosen the assertion."
        );
    }
}

/// **The exercise is not a no-op.** Emptiness is only evidence if something
/// happened first.
///
/// Checked against the tables the write path is *supposed* to fill, so that a
/// change quietly narrowing what a write does cannot make this suite green by
/// doing less. `block` carries most of the weight here: it is the table the
/// Bodies lane added, it is the one deliberately left out of [`UNWRITTEN`] on
/// the grounds that the write path fills it, and this is where that claim is
/// demonstrated rather than asserted in a comment.
#[tokio::test]
async fn the_exercise_actually_writes_something() {
    let world = World::new().await;
    exercise(&world).await;

    for (table, why) in [
        ("entity", "every create lands here"),
        (
            "block",
            "a body divides into blocks and the write path writes them",
        ),
        ("entity_date", "a labelled date was written"),
        ("entity_assignment", "an assignment was written"),
        (
            "entity_part_counter",
            "every part records the counter it moved at",
        ),
        (
            "change",
            "every accepted write appends to the change sequence",
        ),
        ("relation", "a relation was formed"),
        ("intent", "every acknowledgement is held against its intent"),
    ] {
        assert!(
            rows_in(&world, table).await > 0,
            "`{table}` is empty after the exercise, so the emptiness of the \
             deferred tables proves nothing — {why}"
        );
    }
}

/// A typo in [`UNWRITTEN`] would otherwise pass silently.
///
/// `count(*)` over a table that does not exist raises, so the walk above would
/// fail loudly on a misspelling — but only once something reached it. This
/// asks the planner directly, so the list is checked against the store whether
/// or not anything is wrong.
#[tokio::test]
async fn every_table_named_here_exists() {
    let world = World::new().await;
    for (table, _) in UNWRITTEN {
        rows_in(&world, table).await;
    }
    // And the ones deliberately left out are real tables too, so that entry
    // stays an argument about scope rather than a stale name nobody noticed.
    for table in NOT_LISTED {
        rows_in(&world, table).await;
    }
}

/// **The guard is not vacuous.** An assertion that something is empty passes
/// just as well when it is looking in the wrong place, so this plants a row and
/// checks the same helper reports it.
///
/// Written against `entity_version` because it is the entry with the least
/// ambiguity behind it: the scope defers versions in as many words, and nothing
/// anywhere reads the table.
#[tokio::test]
async fn the_check_would_notice_a_row() {
    let world = World::new().await;
    let entity = world.note(&world.one, "something to version").await;
    assert_eq!(rows_in(&world, "entity_version").await, 0);

    sqlx::query(
        "INSERT INTO entity_version (id, entity_id, captured_at, author_member_id, body) \
         VALUES ($1, $2, now(), $3, $4)",
    )
    .bind(Uuid::new_v4())
    .bind(entity.as_uuid())
    .bind(world.one.membership_id())
    .bind("what the note said before")
    .execute(&world.db.pool)
    .await
    .expect("a row can be planted, or this test proves nothing");

    assert_eq!(
        rows_in(&world, "entity_version").await,
        1,
        "the emptiness check cannot see a row, so its passing means nothing"
    );
}
