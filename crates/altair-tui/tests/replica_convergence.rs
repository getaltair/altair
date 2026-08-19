//! A rebuild and an incremental catch-up over the same history end in the same
//! place.
//!
//! **This is the property the whole replica design rests on.** The architecture
//! says a client may always rebuild, so incremental catch-up is an optimisation
//! over `Changes(since=0)` rather than a different mechanism — and an
//! optimisation that produced a different answer would be a silent corruption
//! rather than a slow screen. A change is a statement of what is rather than a
//! delta, which is what makes this true; the test is here because "which is
//! what makes this true" is exactly the kind of reasoning that stops holding
//! when somebody adds a field.
//!
//! It takes no database and no instance: histories are composed here.

use altair_proto::v1;
use altair_tui::device::Store;
use proptest::prelude::*;

/// A small alphabet, so a generated history actually collides: the same entity
/// edited twice, related and then unrelated, gone and created again.
const ENTITIES: usize = 5;
const RELATIONS: usize = 3;

fn id(kind: u8, index: usize) -> Vec<u8> {
    let mut bytes = [0_u8; 16];
    bytes[0] = kind;
    bytes[1] = u8::try_from(index).expect("a small alphabet");
    bytes.to_vec()
}

/// One thing that happened, before it is spelled as a change.
#[derive(Debug, Clone)]
enum Happened {
    /// An entity was created, edited, or became visible to this member.
    Put {
        entity: usize,
        title: u8,
        container: Option<usize>,
        position: u32,
    },
    /// Erased, or no longer visible. Nothing distinguishes the two.
    Gone {
        entity: usize,
    },
    Related {
        relation: usize,
        from: usize,
        to: usize,
    },
    Unrelated {
        relation: usize,
    },
}

fn happening() -> impl Strategy<Value = Happened> {
    prop_oneof![
        (
            0..ENTITIES,
            any::<u8>(),
            proptest::option::of(0..ENTITIES),
            0_u32..4
        )
            .prop_map(|(entity, title, container, position)| Happened::Put {
                entity,
                title,
                container,
                position
            }),
        (0..ENTITIES).prop_map(|entity| Happened::Gone { entity }),
        (0..RELATIONS, 0..ENTITIES, 0..ENTITIES)
            .prop_map(|(relation, from, to)| { Happened::Related { relation, from, to } }),
        (0..RELATIONS).prop_map(|relation| Happened::Unrelated { relation }),
    ]
}

fn as_change(happened: &Happened, position: u64) -> v1::Change {
    let at = Some(v1::Position { value: position });
    let what = match happened {
        Happened::Put {
            entity,
            title,
            container,
            position: within,
        } => v1::change::What::Entity(v1::Entity {
            entity_id: id(b'e', *entity),
            content: Some(v1::EntityContent {
                title: Some(format!("title {title}")),
                category_id: container.map(|c| id(b'e', c)),
                category_position: Some(*within),
                specific: Some(v1::entity_content::Specific::Quest(v1::QuestContent {
                    parent: container.map(|c| v1::quest_content::Parent::ArcId(id(b'e', c))),
                    ladder_position: Some(*within),
                    ..Default::default()
                })),
                ..Default::default()
            }),
            counter: position,
            lifecycle: v1::LifecycleState::Active as i32,
            ..Default::default()
        }),
        Happened::Gone { entity } => v1::change::What::GoneEntityId(id(b'e', *entity)),
        Happened::Related { relation, from, to } => v1::change::What::Relation(v1::Relation {
            relation_id: id(b'r', *relation),
            content: Some(v1::RelationContent {
                from_entity_id: Some(id(b'e', *from)),
                to_entity_id: Some(id(b'e', *to)),
                ..Default::default()
            }),
            ..Default::default()
        }),
        Happened::Unrelated { relation } => v1::change::What::GoneRelationId(id(b'r', *relation)),
    };
    v1::Change {
        position: at,
        what: Some(what),
        changed_block_ids: Vec::new(),
    }
}

/// Everything a screen could ask this store, flattened into something two
/// stores can be compared by.
///
/// Deliberately assembled from what a caller can actually see, rather than by
/// reading the tables: two stores that answer every question identically are
/// the same store as far as anything downstream is concerned, and a difference
/// this cannot see is a difference nothing can.
fn as_answered(store: &Store) -> Vec<String> {
    let mut out = vec![format!("position {}", store.position().expect("position"))];
    for entity in 0..ENTITIES {
        let key = id(b'e', entity);
        let copy = store.instance_copy(&key).expect("read");
        out.push(format!(
            "entity {entity} {:?}",
            copy.map(|entity| (
                entity.content.and_then(|c| c.title),
                entity.counter,
                entity.lifecycle
            ))
        ));
        out.push(format!(
            "beneath {entity} {:?}",
            store
                .beneath(&key)
                .expect("beneath")
                .iter()
                .map(|held| held.entity_id.clone())
                .collect::<Vec<_>>()
        ));
        out.push(format!(
            "within {entity} {:?}",
            store
                .within(&key)
                .expect("within")
                .iter()
                .map(|held| held.entity_id.clone())
                .collect::<Vec<_>>()
        ));
        out.push(format!(
            "touching {entity} {:?}",
            store
                .relations_touching(&key)
                .expect("relations")
                .iter()
                .map(|relation| relation.relation_id.clone())
                .collect::<Vec<_>>()
        ));
    }
    out.push(format!(
        "captures {:?}",
        store
            .captures()
            .expect("captures")
            .iter()
            .map(|held| held.entity_id.clone())
            .collect::<std::collections::BTreeSet<_>>()
    ));
    out
}

fn fresh() -> (tempfile::TempDir, Store) {
    let directory = tempfile::tempdir().expect("a temporary directory");
    let store = Store::open(directory.path()).expect("open");
    (directory, store)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(48))]

    #[test]
    fn a_rebuild_and_a_catch_up_over_one_history_agree(
        history in proptest::collection::vec(happening(), 0..24),
        page in 1_usize..5,
    ) {
        let changes: Vec<v1::Change> = history
            .iter()
            .enumerate()
            .map(|(index, happened)| as_change(happened, index as u64 + 1))
            .collect();
        let last = changes.len() as u64;

        // The rebuild: everything from the beginning, in one answer.
        let (_rebuilt_dir, mut rebuilt) = fresh();
        rebuilt.rebuild().expect("rebuild");
        rebuilt.take_changes(&changes, last).expect("take");

        // The catch-up: the same history, a page at a time, each page taken
        // from wherever the last one left off.
        let (_caught_dir, mut caught) = fresh();
        for chunk in changes.chunks(page) {
            let at = chunk
                .last()
                .and_then(|change| change.position.map(|p| p.value))
                .unwrap_or(0);
            caught.take_changes(chunk, at).expect("take");
        }

        prop_assert_eq!(
            as_answered(&rebuilt),
            as_answered(&caught),
            "a rebuild and an incremental catch-up over the same history must \
             end in the same place; the architecture lets a client choose \
             either at any moment, so a disagreement is a corruption rather \
             than a slow screen"
        );
    }
}
