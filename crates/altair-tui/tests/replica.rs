//! The replica face: instance truth, the overlay over it, and the orders the
//! screens navigate by.
//!
//! Takes no database and no instance. What the change stream would have said is
//! composed here, which is the only way to say "and then somebody else edited
//! it" without a second device.

use altair_proto::v1;
use altair_tui::capture::Captured;
use altair_tui::device::Store;
use altair_tui::wire;

fn store() -> (tempfile::TempDir, Store) {
    let directory = tempfile::tempdir().expect("a temporary directory");
    let store = Store::open(directory.path()).expect("open");
    (directory, store)
}

fn entity(id: &[u8], title: &str, specific: v1::entity_content::Specific) -> v1::Change {
    v1::Change {
        position: None,
        what: Some(v1::change::What::Entity(v1::Entity {
            entity_id: id.to_vec(),
            content: Some(v1::EntityContent {
                title: Some(title.to_string()),
                specific: Some(specific),
                ..Default::default()
            }),
            counter: 1,
            lifecycle: v1::LifecycleState::Active as i32,
            ..Default::default()
        })),
        changed_block_ids: Vec::new(),
    }
}

fn quest(under: &[u8], position: u32) -> v1::entity_content::Specific {
    v1::entity_content::Specific::Quest(v1::QuestContent {
        parent: Some(v1::quest_content::Parent::ArcId(under.to_vec())),
        ladder_position: Some(position),
        ..Default::default()
    })
}

fn relation(id: &[u8], from: &[u8], to: &[u8]) -> v1::Change {
    v1::Change {
        position: None,
        what: Some(v1::change::What::Relation(v1::Relation {
            relation_id: id.to_vec(),
            content: Some(v1::RelationContent {
                from_entity_id: Some(from.to_vec()),
                to_entity_id: Some(to.to_vec()),
                ..Default::default()
            }),
            ..Default::default()
        })),
        changed_block_ids: Vec::new(),
    }
}

#[test]
fn a_ladder_reads_in_the_order_its_container_holds() {
    let (_directory, mut store) = store();
    let arc = wire::fresh_id();
    let (first, second, third) = (wire::fresh_id(), wire::fresh_id(), wire::fresh_id());
    store
        .take_changes(
            &[
                entity(&third, "third", quest(&arc, 2)),
                entity(&first, "first", quest(&arc, 0)),
                entity(&second, "second", quest(&arc, 1)),
            ],
            3,
        )
        .expect("take");

    let ladder: Vec<_> = store
        .beneath(&arc)
        .expect("beneath")
        .iter()
        .map(|held| held.content.title.clone().unwrap_or_default())
        .collect();
    assert_eq!(
        ladder,
        ["first", "second", "third"],
        "an order belongs to the container, not to the thing, and not to the \
         order the changes happened to arrive in"
    );
}

#[test]
fn a_relation_is_read_from_either_end() {
    let (_directory, mut store) = store();
    let (one, two) = (wire::fresh_id(), wire::fresh_id());
    let joining = wire::fresh_id();
    store
        .take_changes(
            &[
                entity(
                    &one,
                    "one",
                    v1::entity_content::Specific::Note(v1::NoteContent::default()),
                ),
                entity(
                    &two,
                    "two",
                    v1::entity_content::Specific::Note(v1::NoteContent::default()),
                ),
                relation(&joining, &one, &two),
            ],
            3,
        )
        .expect("take");

    for (end, name) in [
        (&one, "the end it points from"),
        (&two, "the end it points to"),
    ] {
        let touching = store.relations_touching(end).expect("relations");
        assert_eq!(touching.len(), 1, "{name} finds it");
        assert_eq!(touching[0].relation_id, joining);
    }
}

#[test]
fn an_endpoint_going_takes_the_relations_touching_it() {
    let (_directory, mut store) = store();
    let (one, two) = (wire::fresh_id(), wire::fresh_id());
    let joining = wire::fresh_id();
    store
        .take_changes(
            &[
                entity(
                    &one,
                    "one",
                    v1::entity_content::Specific::Note(v1::NoteContent::default()),
                ),
                entity(
                    &two,
                    "two",
                    v1::entity_content::Specific::Note(v1::NoteContent::default()),
                ),
                relation(&joining, &one, &two),
            ],
            3,
        )
        .expect("take");

    store
        .take_changes(
            &[v1::Change {
                position: None,
                what: Some(v1::change::What::GoneEntityId(one.clone())),
                changed_block_ids: Vec::new(),
            }],
            4,
        )
        .expect("take");

    assert!(
        store
            .relations_touching(&two)
            .expect("relations")
            .is_empty(),
        "a relation reaches a member only when they can see both of its ends, \
         so it must not be readable from the end that is left"
    );
    assert!(
        store
            .relations_touching(&one)
            .expect("relations")
            .is_empty(),
        "and not from the end that went, either"
    );
}

#[test]
fn a_pending_write_stays_visible_across_an_instance_that_cannot_be_reached() {
    let (_directory, mut store) = store();
    let composed = wire::compose_creation(&Captured::Note {
        title: "mine".to_string(),
        body: String::new(),
    });
    store
        .accept_creation(
            &composed.entity_id,
            &composed.content,
            None,
            &composed.intent,
        )
        .expect("accept");

    // The household says something else about the same entity — an earlier
    // version of it, or somebody else's edit. Nothing has taken this device's
    // write yet.
    store
        .take_changes(
            &[entity(
                &composed.entity_id,
                "theirs",
                v1::entity_content::Specific::Note(v1::NoteContent::default()),
            )],
            1,
        )
        .expect("take");

    let held = store
        .held(&composed.entity_id)
        .expect("read")
        .expect("present");
    assert_eq!(
        held.content.title.as_deref(),
        Some("mine"),
        "what the person did on this device is still what they see, because \
         nothing has carried it away yet"
    );
    assert!(held.pending);

    // The instance takes it. From here the household's copy is the one that
    // stands, which is what makes somebody else's later edit arrive at all.
    let outstanding = store.next_to_send(1).expect("outbox").remove(0);
    store
        .applied(&outstanding.intent_id, &composed.entity_id, 1)
        .expect("applied");

    let held = store
        .held(&composed.entity_id)
        .expect("read")
        .expect("present");
    assert!(!held.pending);
    assert_eq!(
        held.content.title.as_deref(),
        Some("theirs"),
        "with nothing of this device's outstanding, the household's copy is the \
         newer one and is what the person is shown"
    );
}

#[test]
fn a_rebuild_keeps_what_has_not_been_sent() {
    let (_directory, mut store) = store();
    let composed = wire::compose_creation(&Captured::Note {
        title: "unsent".to_string(),
        body: String::new(),
    });
    store
        .accept_creation(
            &composed.entity_id,
            &composed.content,
            None,
            &composed.intent,
        )
        .expect("accept");
    store
        .take_changes(
            &[entity(
                &wire::fresh_id(),
                "somebody else's",
                v1::entity_content::Specific::Note(v1::NoteContent::default()),
            )],
            9,
        )
        .expect("take");

    store.rebuild().expect("rebuild");

    assert_eq!(
        store.position().expect("position"),
        0,
        "back to the beginning"
    );
    assert_eq!(
        store.captures().expect("captures").len(),
        1,
        "the copy went and the capture did not: an unsent capture is not the \
         instance's to have an opinion about"
    );
    assert_eq!(
        store.next_to_send(10).expect("outbox").len(),
        1,
        "and it is still on its way"
    );
}

#[test]
fn both_sides_of_a_retained_conflict_are_kept() {
    let (_directory, mut store) = store();
    let id = wire::fresh_id();
    let mut change = entity(
        &id,
        "theirs",
        v1::entity_content::Specific::Note(v1::NoteContent::default()),
    );
    if let Some(v1::change::What::Entity(held)) = change.what.as_mut() {
        held.conflicts = vec![v1::ConflictedPart {
            part: Some(v1::conflicted_part::Part::ContentField(1)),
            mine: Some(v1::ConflictSide {
                member_id: wire::fresh_id(),
                value: "mine".to_string(),
            }),
            theirs: Some(v1::ConflictSide {
                member_id: wire::fresh_id(),
                value: "theirs".to_string(),
            }),
        }];
    }
    store.take_changes(&[change], 1).expect("take");

    let copy = store.instance_copy(&id).expect("read").expect("present");
    assert_eq!(copy.conflicts.len(), 1, "the conflict came through");
    let conflict = &copy.conflicts[0];
    assert_eq!(
        conflict.mine.as_ref().map(|s| s.value.as_str()),
        Some("mine")
    );
    assert_eq!(
        conflict.theirs.as_ref().map(|s| s.value.as_str()),
        Some("theirs"),
        "both values are retained and whose each is is a fact the person needs; \
         the entity is readable throughout and nothing was sent back"
    );
    assert!(
        store.held(&id).expect("read").is_some(),
        "and it is not a failure: the entity is present, and a conflict is \
         something to look at rather than something to fix before continuing"
    );
}
