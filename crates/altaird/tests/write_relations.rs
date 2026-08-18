//! Relations: create, edit, remove, restore, and removal by erasure of an end.
//!
//! The load-bearing assertion in this file is
//! [`behaviour_follows_the_declaration_and_not_a_branch`]. Every relation type
//! declared here is one the shipped set has never contained, so a hardcoded
//! branch on *blocks*, *uses*, or *references* would fail every one of them.

mod common;

use altair_proto::v1;
use altaird::store::ids::EntityId;
use common::*;
use sqlx::Row;
use uuid::Uuid;

struct Ends {
    from: Uuid,
    to: Uuid,
    quantity: Option<String>,
    resolution: Option<String>,
}

async fn ends(w: &World, id: Uuid) -> Ends {
    let r = sqlx::query(
        "SELECT from_entity_id, to_entity_id, quantity::text AS q, \
         resolution::text AS r FROM relation WHERE id = $1",
    )
    .bind(id)
    .fetch_one(&w.db.pool)
    .await
    .expect("relation");
    Ends {
        from: r.try_get("from_entity_id").unwrap(),
        to: r.try_get("to_entity_id").unwrap(),
        quantity: r.try_get("q").unwrap(),
        resolution: r.try_get("r").unwrap(),
    }
}

async fn two_notes(w: &World) -> (EntityId, EntityId) {
    (w.note(&w.one, "a").await, w.note(&w.one, "b").await)
}

// --- creating ------------------------------------------------------------

#[tokio::test]
async fn a_relation_joins_two_entities_and_is_one_record() {
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;
    let ack = w
        .submit(
            &w.one,
            create_relation(Uuid::new_v4(), relation_between(a, b)),
        )
        .await;

    let id = relation_id(&ack);
    assert_eq!(w.relation_lifecycle(id).await.as_deref(), Some("active"));
    assert_eq!(
        w.changes().await.last().expect("a change").kind,
        "relation_written"
    );
}

#[tokio::test]
async fn an_untyped_relation_is_put_in_canonical_order() {
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;
    let (low, high) = if a.as_uuid() < b.as_uuid() {
        (a, b)
    } else {
        (b, a)
    };

    // Submitted the wrong way round.
    let ack = w
        .submit(
            &w.one,
            create_relation(Uuid::new_v4(), relation_between(high, low)),
        )
        .await;
    let e = ends(&w, relation_id(&ack)).await;
    assert_eq!(
        (e.from, e.to),
        (low.as_uuid(), high.as_uuid()),
        "neither end of an untyped relation is privileged, so the same \
         connection cannot be recorded twice"
    );
}

#[tokio::test]
async fn a_symmetric_type_is_put_in_canonical_order_and_an_asymmetric_one_is_not() {
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;
    let (low, high) = if a.as_uuid() < b.as_uuid() {
        (a, b)
    } else {
        (b, a)
    };

    let symmetric = w.declare_type("sits beside", false, false).await;
    let asymmetric = w.declare_type("outranks", true, false).await;

    let ack = w
        .submit(
            &w.one,
            create_relation(
                Uuid::new_v4(),
                v1::RelationContent {
                    relation_type_id: Some(symmetric.as_bytes().to_vec()),
                    ..relation_between(high, low)
                },
            ),
        )
        .await;
    let e = ends(&w, relation_id(&ack)).await;
    assert_eq!((e.from, e.to), (low.as_uuid(), high.as_uuid()));

    let ack = w
        .submit(
            &w.one,
            create_relation(
                Uuid::new_v4(),
                v1::RelationContent {
                    relation_type_id: Some(asymmetric.as_bytes().to_vec()),
                    ..relation_between(high, low)
                },
            ),
        )
        .await;
    let e = ends(&w, relation_id(&ack)).await;
    assert_eq!(
        (e.from, e.to),
        (high.as_uuid(), low.as_uuid()),
        "direction is a property of an asymmetric relation and must survive"
    );
}

#[tokio::test]
async fn behaviour_follows_the_declaration_and_not_a_branch() {
    // A type the shipped set has never contained. If anything anywhere matched
    // on a type's label, this would get neither behaviour.
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;
    let invented = w.declare_type("borrows from", true, true).await;

    let ack = w
        .submit(
            &w.one,
            create_relation(
                Uuid::new_v4(),
                v1::RelationContent {
                    relation_type_id: Some(invented.as_bytes().to_vec()),
                    uses: Some(v1::UsesProperties {
                        quantity: Some(v1::Decimal {
                            units: 2,
                            nanos: 500_000_000,
                        }),
                        resolution: v1::UsesResolution::Consumed as i32,
                    }),
                    ..relation_between(b, a)
                },
            ),
        )
        .await;

    let e = ends(&w, relation_id(&ack)).await;
    assert_eq!(
        (e.from, e.to),
        (b.as_uuid(), a.as_uuid()),
        "an invented asymmetric type gets the asymmetric reading"
    );
    assert_eq!(
        e.quantity.as_deref(),
        Some("2.500000000"),
        "an invented quantified type gets to carry a quantity"
    );
    assert_eq!(e.resolution.as_deref(), Some("consumed"));
}

#[tokio::test]
async fn a_negative_quantity_keeps_its_sign() {
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;
    let quantified = w.declare_type("owes", true, true).await;
    let ack = w
        .submit(
            &w.one,
            create_relation(
                Uuid::new_v4(),
                v1::RelationContent {
                    relation_type_id: Some(quantified.as_bytes().to_vec()),
                    uses: Some(v1::UsesProperties {
                        quantity: Some(v1::Decimal {
                            units: -3,
                            nanos: -250_000_000,
                        }),
                        resolution: v1::UsesResolution::Unresolved as i32,
                    }),
                    ..relation_between(a, b)
                },
            ),
        )
        .await;
    assert_eq!(
        ends(&w, relation_id(&ack)).await.quantity.as_deref(),
        Some("-3.250000000"),
        "negative availability is meaningful information about a cupboard"
    );
}

#[tokio::test]
async fn a_property_is_permitted_only_where_the_type_declares_it() {
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;
    let plain = w.declare_type("mentions", true, false).await;

    let uses = Some(v1::UsesProperties {
        quantity: Some(v1::Decimal { units: 1, nanos: 0 }),
        resolution: v1::UsesResolution::Unresolved as i32,
    });

    // On a type that declares none.
    let ack = w
        .submit(
            &w.one,
            create_relation(
                Uuid::new_v4(),
                v1::RelationContent {
                    relation_type_id: Some(plain.as_bytes().to_vec()),
                    uses,
                    ..relation_between(a, b)
                },
            ),
        )
        .await;
    assert_eq!(refused(&ack).reason, v1::RefusalReason::Malformed as i32);

    // And on no type at all. An untyped relation carries nothing.
    let ack = w
        .submit(
            &w.one,
            create_relation(
                Uuid::new_v4(),
                v1::RelationContent {
                    uses,
                    ..relation_between(a, b)
                },
            ),
        )
        .await;
    assert_eq!(refused(&ack).reason, v1::RefusalReason::Malformed as i32);
}

#[tokio::test]
async fn a_duplicate_untyped_unanchored_relation_is_refused_either_way_round() {
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;
    w.submit(
        &w.one,
        create_relation(Uuid::new_v4(), relation_between(a, b)),
    )
    .await;

    let same = w
        .submit(
            &w.one,
            create_relation(Uuid::new_v4(), relation_between(a, b)),
        )
        .await;
    assert_eq!(refused(&same).reason, v1::RefusalReason::Malformed as i32);

    let reversed = w
        .submit(
            &w.one,
            create_relation(Uuid::new_v4(), relation_between(b, a)),
        )
        .await;
    assert_eq!(
        refused(&reversed).reason,
        v1::RefusalReason::Malformed as i32,
        "an untyped unanchored relation is the same connection however it is \
         submitted"
    );
}

#[tokio::test]
async fn a_typed_relation_is_not_a_duplicate_of_an_untyped_one() {
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;
    let kind = w.declare_type("annotates", true, false).await;

    w.submit(
        &w.one,
        create_relation(Uuid::new_v4(), relation_between(a, b)),
    )
    .await;
    let typed = w
        .submit(
            &w.one,
            create_relation(
                Uuid::new_v4(),
                v1::RelationContent {
                    relation_type_id: Some(kind.as_bytes().to_vec()),
                    ..relation_between(a, b)
                },
            ),
        )
        .await;
    assert!(matches!(
        typed.outcome,
        Some(v1::acknowledgement::Outcome::Applied(_))
    ));
}

#[tokio::test]
async fn a_relation_needs_both_ends_visible() {
    let w = World::new().await;
    let mine = w.note(&w.one, "mine").await;
    let theirs = w.note(&w.two, "theirs").await;
    let ack = w
        .submit(
            &w.one,
            create_relation(Uuid::new_v4(), relation_between(mine, theirs)),
        )
        .await;
    assert!(
        is_not_available(&ack),
        "a relation whose far end a member cannot see would show them that \
         something is there"
    );
}

#[tokio::test]
async fn a_relation_does_not_join_an_entity_to_itself() {
    let w = World::new().await;
    let a = w.note(&w.one, "a").await;
    let ack = w
        .submit(
            &w.one,
            create_relation(Uuid::new_v4(), relation_between(a, a)),
        )
        .await;
    assert_eq!(refused(&ack).reason, v1::RefusalReason::Malformed as i32);
}

#[tokio::test]
async fn an_unknown_relation_type_is_nothing() {
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;
    let ack = w
        .submit(
            &w.one,
            create_relation(
                Uuid::new_v4(),
                v1::RelationContent {
                    relation_type_id: Some(Uuid::new_v4().as_bytes().to_vec()),
                    ..relation_between(a, b)
                },
            ),
        )
        .await;
    assert!(is_not_available(&ack));
}

// --- editing -------------------------------------------------------------

#[tokio::test]
async fn a_relation_edit_is_unconditional() {
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;
    let kind = w.declare_type("supersedes", true, false).await;
    let created = w
        .submit(
            &w.one,
            create_relation(Uuid::new_v4(), relation_between(a, b)),
        )
        .await;
    let id = relation_id(&created);

    // No counter, so no base to be stale against: relations carry no write
    // counter anywhere in the documents, which the proto records as gap (a).
    let ack = w
        .submit(
            &w.one,
            edit_relation(
                id,
                v1::RelationContent {
                    relation_type_id: Some(kind.as_bytes().to_vec()),
                    ..relation_between(a, b)
                },
            ),
        )
        .await;
    assert!(matches!(
        ack.outcome,
        Some(v1::acknowledgement::Outcome::Applied(_))
    ));
}

// --- removal, restoration, erasure --------------------------------------

#[tokio::test]
async fn removing_a_relation_is_reversible_and_it_comes_back_with_the_act() {
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;
    let id = relation_id(
        &w.submit(
            &w.one,
            create_relation(Uuid::new_v4(), relation_between(a, b)),
        )
        .await,
    );

    // One act removes the entity and the connection together.
    w.submit(&w.one, remove(&[a], &[id])).await;
    assert_eq!(w.relation_lifecycle(id).await.as_deref(), Some("deleted"));
    assert_eq!(
        w.changes().await.last().expect("a change").kind,
        "relation_gone",
        "the wire's Relation carries no lifecycle, so a removed one is gone"
    );

    let ack = w.submit(&w.one, restore(a, true)).await;
    assert_eq!(applied(&ack).relation_ids.len(), 1);
    assert_eq!(w.relation_lifecycle(id).await.as_deref(), Some("active"));
    assert_eq!(
        w.changes().await.last().expect("a change").kind,
        "relation_written"
    );
}

#[tokio::test]
async fn a_relation_does_not_come_back_on_its_own() {
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;
    let id = relation_id(
        &w.submit(
            &w.one,
            create_relation(Uuid::new_v4(), relation_between(a, b)),
        )
        .await,
    );
    w.submit(&w.one, remove(&[a], &[id])).await;

    // Restoring the entity without its act leaves the connection removed.
    w.submit(&w.one, restore(a, false)).await;
    assert_eq!(
        w.relation_lifecycle(id).await.as_deref(),
        Some("deleted"),
        "there is no restoring a connection on its own, and nothing lists \
         removed connections for a person to notice one missing from"
    );
}

#[tokio::test]
async fn a_removed_connection_can_be_formed_again() {
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;
    let id = relation_id(
        &w.submit(
            &w.one,
            create_relation(Uuid::new_v4(), relation_between(a, b)),
        )
        .await,
    );
    w.submit(&w.one, remove(&[], &[id])).await;

    let again = w
        .submit(
            &w.one,
            create_relation(Uuid::new_v4(), relation_between(a, b)),
        )
        .await;
    assert!(
        matches!(
            again.outcome,
            Some(v1::acknowledgement::Outcome::Applied(_))
        ),
        "one sitting in holding is not a connection that exists, and forming \
         it again is the person's answer to having removed it"
    );
}

#[tokio::test]
async fn erasing_either_endpoint_removes_the_relation_outright() {
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;
    let id = relation_id(
        &w.submit(
            &w.one,
            create_relation(Uuid::new_v4(), relation_between(a, b)),
        )
        .await,
    );

    let ack = w.submit(&w.one, erase(&[b])).await;
    assert_eq!(applied(&ack).relation_ids.len(), 1);
    assert_eq!(
        w.relation_lifecycle(id).await,
        None,
        "erasing either endpoint removes the relation outright, which is what \
         erasure means everywhere"
    );
    let changes = w.changes().await;
    assert!(
        changes
            .iter()
            .any(|c| c.kind == "relation_gone" && c.relation == Some(id)),
        "the sequence has to be able to speak about something that no longer exists"
    );
}

#[tokio::test]
async fn removing_a_relation_this_member_cannot_see_converges() {
    let w = World::new().await;
    let (a, b) = (w.note(&w.two, "a").await, w.note(&w.two, "b").await);
    let id = relation_id(
        &w.submit(
            &w.two,
            create_relation(Uuid::new_v4(), relation_between(a, b)),
        )
        .await,
    );

    let ack = w.submit(&w.one, remove(&[], &[id])).await;
    assert!(matches!(
        ack.outcome,
        Some(v1::acknowledgement::Outcome::Applied(_))
    ));
    assert!(applied(&ack).relation_ids.is_empty());
    assert_eq!(w.relation_lifecycle(id).await.as_deref(), Some("active"));
}

// --- what a cross-model review found -------------------------------------

#[tokio::test]
async fn an_edit_cannot_make_a_relation_into_a_duplicate() {
    // Two relations between one pair are legitimate while one of them carries a
    // type. Clearing that type leaves two untyped unanchored records of one
    // connection, which the create path refuses and the edit path used not to.
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;
    let kind = w.declare_type("elaborates", true, false).await;

    w.submit(
        &w.one,
        create_relation(Uuid::new_v4(), relation_between(a, b)),
    )
    .await;
    let typed = relation_id(
        &w.submit(
            &w.one,
            create_relation(
                Uuid::new_v4(),
                v1::RelationContent {
                    relation_type_id: Some(kind.as_bytes().to_vec()),
                    ..relation_between(a, b)
                },
            ),
        )
        .await,
    );

    let ack = w
        .submit(&w.one, edit_relation(typed, relation_between(a, b)))
        .await;
    assert_eq!(refused(&ack).reason, v1::RefusalReason::Malformed as i32);
}

#[tokio::test]
async fn an_edit_that_changes_nothing_does_not_refuse_itself() {
    // The duplicate check has to exclude the relation being edited, or every
    // no-op edit of an untyped relation collides with itself.
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;
    let id = relation_id(
        &w.submit(
            &w.one,
            create_relation(Uuid::new_v4(), relation_between(a, b)),
        )
        .await,
    );
    let ack = w
        .submit(&w.one, edit_relation(id, relation_between(a, b)))
        .await;
    assert!(matches!(
        ack.outcome,
        Some(v1::acknowledgement::Outcome::Applied(_))
    ));
}

#[tokio::test]
async fn a_decimal_that_cannot_be_read_is_refused_rather_than_reinterpreted() {
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;
    let quantified = w.declare_type("commits", true, true).await;

    // A fraction of a whole unit or more. Rendered into a fixed nine-place
    // field this became 1.15, which is a different number stated confidently.
    // And opposite signs, which the wire forbids and which read as either 0.75
    // or -1.25 depending on which half you believe.
    for bad in [
        v1::Decimal {
            units: 1,
            nanos: 1_500_000_000,
        },
        v1::Decimal {
            units: 1,
            nanos: -250_000_000,
        },
        v1::Decimal {
            units: -1,
            nanos: 250_000_000,
        },
    ] {
        let ack = w
            .submit(
                &w.one,
                create_relation(
                    Uuid::new_v4(),
                    v1::RelationContent {
                        relation_type_id: Some(quantified.as_bytes().to_vec()),
                        uses: Some(v1::UsesProperties {
                            quantity: Some(bad),
                            resolution: v1::UsesResolution::Unresolved as i32,
                        }),
                        ..relation_between(a, b)
                    },
                ),
            )
            .await;
        assert_eq!(
            refused(&ack).reason,
            v1::RefusalReason::Malformed as i32,
            "{bad:?} was accepted and rendered as something else"
        );
    }
}

#[tokio::test]
async fn a_relation_create_naming_an_invisible_identity_is_refused_and_never_faults() {
    let w = World::new().await;
    let (theirs_a, theirs_b) = (w.note(&w.two, "a").await, w.note(&w.two, "b").await);
    let taken = relation_id(
        &w.submit(
            &w.two,
            create_relation(Uuid::new_v4(), relation_between(theirs_a, theirs_b)),
        )
        .await,
    );

    let (mine_a, mine_b) = two_notes(&w).await;
    let acks = w
        .write
        .submit(
            &w.one,
            &[v1::Intent {
                intent_id: Uuid::new_v4().as_bytes().to_vec(),
                action: Some(create_relation(taken, relation_between(mine_a, mine_b))),
            }],
        )
        .await
        .expect("an identifier somebody else holds is an answer, not a fault");
    assert!(is_not_available(&acks[0]));
}

// --- anchors -------------------------------------------------------------
//
// An anchor records where a relation was formed. It belongs to the relation and
// not to the body, so the body stays plain text and removing the relation
// rewrites nothing. It is not a type-declared property and needs no type.
//
// One sentence of the substrate decides most of what follows: *"Removing a
// person's connection as a side effect of them editing a sentence is not
// something the system does."* The anchor is the weakest thing a relation
// carries, so wherever something has to give, the anchor gives and the relation
// does not.

/// A note whose body is one entry per line, and the identities of its blocks.
async fn note_with_blocks(w: &World, body: &str) -> (EntityId, Vec<Uuid>) {
    let id = w
        .create(
            &w.one,
            v1::entity_content::Specific::Note(v1::NoteContent {
                body: Some(body.into()),
                cleared: Vec::new(),
            }),
            v1::EntityContent {
                audience_member_ids: vec![
                    w.one.membership_id().as_bytes().to_vec(),
                    w.two.membership_id().as_bytes().to_vec(),
                ],
                ..Default::default()
            },
        )
        .await;
    let blocks = sqlx::query("SELECT id FROM block WHERE entity_id = $1 ORDER BY position")
        .bind(id.as_uuid())
        .fetch_all(&w.db.pool)
        .await
        .expect("blocks")
        .iter()
        .map(|r| r.try_get("id").unwrap())
        .collect();
    (id, blocks)
}

fn at(entity: EntityId, block: Uuid, phrase: &str) -> v1::Anchor {
    v1::Anchor {
        entity_id: entity.as_uuid().as_bytes().to_vec(),
        block_id: block.as_bytes().to_vec(),
        phrase: phrase.into(),
    }
}

/// The three columns an anchor lives in.
async fn anchor_of(w: &World, relation: Uuid) -> (Option<Uuid>, Option<Uuid>, Option<String>) {
    let r = sqlx::query(
        "SELECT anchor_entity_id, anchor_block_id, anchor_phrase FROM relation WHERE id = $1",
    )
    .bind(relation)
    .fetch_one(&w.db.pool)
    .await
    .expect("relation");
    (
        r.try_get("anchor_entity_id").unwrap(),
        r.try_get("anchor_block_id").unwrap(),
        r.try_get("anchor_phrase").unwrap(),
    )
}

async fn anchored(
    w: &World,
    from: EntityId,
    to: EntityId,
    anchor: v1::Anchor,
) -> v1::Acknowledgement {
    w.submit(
        &w.one,
        create_relation(
            Uuid::new_v4(),
            v1::RelationContent {
                anchor: Some(anchor),
                ..relation_between(from, to)
            },
        ),
    )
    .await
}

/// An anchor at the block itself. An empty phrase is what the wire says that
/// is, and it is stored as null rather than as an empty string — a phrase
/// nobody wrote is not a phrase.
#[tokio::test]
async fn an_anchor_at_a_block_records_the_block_and_no_phrase() {
    let w = World::new().await;
    let (note, blocks) = note_with_blocks(&w, "- eggs\n- milk\n").await;
    let other = w.note(&w.one, "the other end").await;

    let ack = anchored(&w, note, other, at(note, blocks[0], "")).await;
    let id = relation_id(&ack);

    assert_eq!(
        anchor_of(&w, id).await,
        (Some(note.as_uuid()), Some(blocks[0]), None)
    );
}

/// An anchor at a phrase within a block records the phrase as well.
///
/// **No type is involved.** An anchor is not a type-declared property and does
/// not require a type, which is the one place the anchor rules and the
/// properties rules differ.
#[tokio::test]
async fn an_anchor_at_a_phrase_records_it_and_needs_no_type() {
    let w = World::new().await;
    let (note, blocks) = note_with_blocks(&w, "buy eggs on the way home\n").await;
    let other = w.note(&w.one, "the other end").await;

    let ack = anchored(&w, note, other, at(note, blocks[0], "eggs")).await;
    let id = relation_id(&ack);

    assert_eq!(
        anchor_of(&w, id).await,
        (Some(note.as_uuid()), Some(blocks[0]), Some("eggs".into()))
    );
    // Untyped, and none the worse for it.
    let typed: Option<Uuid> =
        sqlx::query("SELECT relation_type_id AS t FROM relation WHERE id = $1")
            .bind(id)
            .fetch_one(&w.db.pool)
            .await
            .expect("relation")
            .try_get("t")
            .unwrap();
    assert!(typed.is_none());
}

/// An anchor locates into one of the two entities the relation joins.
///
/// Malformed with a detail, because it is a statement about the message: the
/// client already holds both endpoint identifiers, so saying so tells it
/// nothing it did not send.
#[tokio::test]
async fn an_anchor_into_an_entity_the_relation_does_not_join_is_malformed() {
    let w = World::new().await;
    let (elsewhere, blocks) = note_with_blocks(&w, "- eggs\n").await;
    let (a, b) = two_notes(&w).await;

    let ack = anchored(&w, a, b, at(elsewhere, blocks[0], "")).await;
    let detail = &refused(&ack).detail;
    assert_eq!(refused(&ack).reason, v1::RefusalReason::Malformed as i32);
    assert!(detail.contains("one of the two entities"), "{detail}");
}

/// An anchor attaches at a block or at a phrase within one. An entity alone is
/// neither, and is refused rather than admitted as a third kind.
///
/// The store *holds* that shape, because it is the residue a removed block
/// leaves. Produced by the instance, never submitted by a client.
#[tokio::test]
async fn an_anchor_naming_no_block_is_malformed() {
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;

    let ack = anchored(
        &w,
        a,
        b,
        v1::Anchor {
            entity_id: a.as_uuid().as_bytes().to_vec(),
            block_id: Vec::new(),
            phrase: String::new(),
        },
    )
    .await;
    let detail = &refused(&ack).detail;
    assert_eq!(refused(&ack).reason, v1::RefusalReason::Malformed as i32);
    assert!(detail.contains("names no block"), "{detail}");
}

/// **A block of something else and a block of nothing are one answer.**
///
/// A block belongs to an entity, so an acknowledgement that distinguished the
/// two would confirm that an entity exists — the disclosure the single refusal
/// reason exists to prevent, arriving through a field nobody thinks of as
/// naming an entity.
#[tokio::test]
async fn a_block_of_something_else_is_the_same_nothing_as_a_block_of_nothing() {
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;
    let (elsewhere, elsewhere_blocks) = note_with_blocks(&w, "- eggs\n").await;
    let _ = elsewhere;

    let nothing = anchored(&w, a, b, at(a, Uuid::new_v4(), "")).await;
    let elsewhere = anchored(&w, a, b, at(a, elsewhere_blocks[0], "")).await;

    assert!(is_not_available(&nothing));
    assert!(is_not_available(&elsewhere));
    assert_eq!(
        refused(&nothing).detail,
        refused(&elsewhere).detail,
        "the two answers differ, and the difference is an oracle"
    );
    assert!(refused(&nothing).detail.is_empty());
}

/// **The decision worth reading twice: a phrase that is no longer there does
/// not refuse the connection away.**
///
/// A client submits an anchor against the body it last saw. Somebody reworded
/// that sentence meanwhile. Refusing would destroy the whole connection
/// permanently — a refusal is an answer and the outbox holds no retry for it —
/// because somebody else edited a sentence, which is a worse version of the
/// thing the substrate prohibits. So the anchor gives way and the relation is
/// formed without one.
#[tokio::test]
async fn a_phrase_that_is_no_longer_there_forms_the_relation_without_an_anchor() {
    let w = World::new().await;
    let (note, blocks) = note_with_blocks(&w, "buy eggs on the way home\n").await;
    let other = w.note(&w.one, "the other end").await;

    // Somebody else rewords the sentence the anchor was formed against.
    let base = w.counter(note).await;
    w.submit(
        &w.two,
        edit_entity(
            note,
            base as u64,
            v1::EntityContent {
                specific: Some(v1::entity_content::Specific::Note(v1::NoteContent {
                    body: Some("buy bread on the way home\n".into()),
                    cleared: Vec::new(),
                })),
                ..Default::default()
            },
        ),
    )
    .await;

    let ack = anchored(&w, note, other, at(note, blocks[0], "eggs")).await;
    let id = relation_id(&ack);

    // Formed, not refused...
    assert_eq!(w.relation_lifecycle(id).await.as_deref(), Some("active"));
    // ...and the block goes with the phrase rather than the anchor falling back
    // to it. Where it was formed is still true and stays.
    assert_eq!(anchor_of(&w, id).await, (Some(note.as_uuid()), None, None));
}

/// An edit carries an anchor, and an edit that does not carry one clears it.
///
/// A relation's content has no untouched state: there is no base counter to
/// scope a write against and no part machinery over a relation, so an edit is
/// the whole record. That was already true of the type and of a quantity, and
/// the anchor joins them rather than acquiring a rule of its own.
#[tokio::test]
async fn an_edit_carries_the_anchor_and_an_edit_without_one_clears_it() {
    let w = World::new().await;
    let (note, blocks) = note_with_blocks(&w, "- eggs\n- milk\n").await;
    let other = w.note(&w.one, "the other end").await;

    let id = relation_id(&anchored(&w, note, other, at(note, blocks[0], "")).await);

    // Moved to the second entry.
    w.submit(
        &w.one,
        edit_relation(
            id,
            v1::RelationContent {
                anchor: Some(at(note, blocks[1], "milk")),
                ..relation_between(note, other)
            },
        ),
    )
    .await;
    assert_eq!(
        anchor_of(&w, id).await,
        (Some(note.as_uuid()), Some(blocks[1]), Some("milk".into()))
    );

    // And an edit carrying no anchor is an edit to a relation with no anchor.
    w.submit(&w.one, edit_relation(id, relation_between(note, other)))
        .await;
    assert_eq!(anchor_of(&w, id).await, (None, None, None));
}

/// An anchored relation is not a duplicate of an unanchored one. Where a
/// connection was formed is part of what distinguishes it.
#[tokio::test]
async fn an_anchored_relation_is_not_a_duplicate_of_an_unanchored_one() {
    let w = World::new().await;
    let (note, blocks) = note_with_blocks(&w, "- eggs\n").await;
    let other = w.note(&w.one, "the other end").await;

    w.submit(
        &w.one,
        create_relation(Uuid::new_v4(), relation_between(note, other)),
    )
    .await;
    let ack = anchored(&w, note, other, at(note, blocks[0], "")).await;
    applied(&ack);
}

/// **Removing a block never fails a body write, beside an otherwise identical
/// unanchored relation.**
///
/// Migration one declines a unique index over the unanchored pair for exactly
/// this shape: a relation that loses its anchor becomes unanchored, collides
/// with one already recorded between the same pair, and the block removal
/// fails. *"A body edit must never fail because a relation lost an anchor, and
/// that outranks refusing a duplicate structurally."*
///
/// Two things keep that true here, and the weaker one is the one to rely on.
/// The relation does not become *fully* unanchored — where it was formed
/// survives the block, so a residue remains and `is_duplicate` counts a residue
/// as an anchor. But the guarantee does not rest on that: the removal path
/// carries no duplicate check at all, so a later change to what counts as
/// anchored cannot reintroduce the failure.
#[tokio::test]
async fn removing_a_block_never_fails_beside_an_identical_unanchored_relation() {
    let w = World::new().await;
    let (note, blocks) = note_with_blocks(&w, "- eggs\n- milk\n").await;
    let other = w.note(&w.one, "the other end").await;

    // The unanchored one it will collide with, and the anchored one.
    w.submit(
        &w.one,
        create_relation(Uuid::new_v4(), relation_between(note, other)),
    )
    .await;
    let anchored_id = relation_id(&anchored(&w, note, other, at(note, blocks[0], "")).await);

    // Remove the block the second is anchored to.
    let base = w.counter(note).await;
    let ack = w
        .submit(
            &w.one,
            edit_entity(
                note,
                base as u64,
                v1::EntityContent {
                    specific: Some(v1::entity_content::Specific::Note(v1::NoteContent {
                        body: Some("- milk\n".into()),
                        cleared: Vec::new(),
                    })),
                    ..Default::default()
                },
            ),
        )
        .await;
    applied(&ack);

    // Both relations are still here, and the second is now unanchored except
    // for where it was formed.
    assert_eq!(
        w.relation_lifecycle(anchored_id).await.as_deref(),
        Some("active")
    );
    assert_eq!(
        anchor_of(&w, anchored_id).await,
        (Some(note.as_uuid()), None, None)
    );
}

/// A block anchor holds while its block remains, including through edits to
/// the block's own text. This is the half of block identity that anchors rely
/// on: rewording an entry does not detach a relation anchored to the entry.
#[tokio::test]
async fn a_block_anchor_holds_through_a_rewording_of_its_block() {
    let w = World::new().await;
    let (note, blocks) = note_with_blocks(&w, "- eggs\n- milk\n").await;
    let other = w.note(&w.one, "the other end").await;
    let id = relation_id(&anchored(&w, note, other, at(note, blocks[0], "")).await);

    let base = w.counter(note).await;
    w.submit(
        &w.one,
        edit_entity(
            note,
            base as u64,
            v1::EntityContent {
                specific: Some(v1::entity_content::Specific::Note(v1::NoteContent {
                    body: Some("- half a dozen eggs\n- milk\n".into()),
                    cleared: Vec::new(),
                })),
                ..Default::default()
            },
        ),
    )
    .await;

    assert_eq!(
        anchor_of(&w, id).await,
        (Some(note.as_uuid()), Some(blocks[0]), None),
        "rewording an entry detached a relation anchored to the entry itself"
    );
}

/// **A phrase anchor is lost when its text is edited, and the relation
/// survives without an anchor.**
///
/// The counterpart of the test above, and the one that separates the two kinds:
/// the same edit that a block anchor rides out takes a phrase anchor with it,
/// because the words the person pointed at are no longer there. The block goes
/// with the phrase rather than the anchor falling back to it — both places the
/// substrate states this say *without an anchor*, and re-pointing somebody's
/// connection at the whole paragraph would be choosing a place they never did.
#[tokio::test]
async fn a_phrase_anchor_is_lost_when_its_text_is_edited() {
    let w = World::new().await;
    let (note, blocks) = note_with_blocks(&w, "buy eggs on the way home\n").await;
    let other = w.note(&w.one, "the other end").await;
    let id = relation_id(&anchored(&w, note, other, at(note, blocks[0], "eggs")).await);

    let base = w.counter(note).await;
    w.submit(
        &w.one,
        edit_entity(
            note,
            base as u64,
            v1::EntityContent {
                specific: Some(v1::entity_content::Specific::Note(v1::NoteContent {
                    body: Some("buy bread on the way home\n".into()),
                    cleared: Vec::new(),
                })),
                ..Default::default()
            },
        ),
    )
    .await;

    assert_eq!(w.relation_lifecycle(id).await.as_deref(), Some("active"));
    assert_eq!(anchor_of(&w, id).await, (Some(note.as_uuid()), None, None));
}

/// An edit that leaves the phrase standing leaves the anchor standing. The rule
/// is that the anchor's *text* survives, not that the block was untouched.
#[tokio::test]
async fn a_phrase_anchor_survives_an_edit_that_leaves_its_words_alone() {
    let w = World::new().await;
    let (note, blocks) = note_with_blocks(&w, "buy eggs on the way home\n").await;
    let other = w.note(&w.one, "the other end").await;
    let id = relation_id(&anchored(&w, note, other, at(note, blocks[0], "eggs")).await);

    let base = w.counter(note).await;
    w.submit(
        &w.one,
        edit_entity(
            note,
            base as u64,
            v1::EntityContent {
                specific: Some(v1::entity_content::Specific::Note(v1::NoteContent {
                    body: Some("buy eggs on the way back\n".into()),
                    cleared: Vec::new(),
                })),
                ..Default::default()
            },
        ),
    )
    .await;

    assert_eq!(
        anchor_of(&w, id).await,
        (Some(note.as_uuid()), Some(blocks[0]), Some("eggs".into())),
        "an edit elsewhere in the sentence took a phrase anchor with it"
    );
}

/// **Where two people edit one block and both values are retained, anchors are
/// judged against the version that landed.**
///
/// *"Where they edit the same block and one version is chosen, an anchor whose
/// text survives holds, and one whose text does not leaves the relation intact
/// without an anchor."* Both halves in one write: the phrase that is still in
/// the stored text keeps its anchor, the phrase that is not loses it, and the
/// conflict over the block is recorded either way.
#[tokio::test]
async fn after_a_conflict_over_one_block_the_surviving_phrase_keeps_its_anchor() {
    let w = World::new().await;
    let (note, blocks) = note_with_blocks(&w, "buy eggs and bread today\n").await;
    let other = w.note(&w.one, "the other end").await;

    let eggs = relation_id(&anchored(&w, note, other, at(note, blocks[0], "eggs")).await);
    let today = relation_id(&anchored(&w, note, other, at(note, blocks[0], "today")).await);
    let base = w.counter(note).await;

    let reword = |body: &str| {
        edit_entity(
            note,
            base as u64,
            v1::EntityContent {
                specific: Some(v1::entity_content::Specific::Note(v1::NoteContent {
                    body: Some(body.into()),
                    cleared: Vec::new(),
                })),
                ..Default::default()
            },
        )
    };

    w.submit(&w.one, reword("buy eggs and milk today\n")).await;
    let ack = w.submit(&w.two, reword("buy bread and milk today\n")).await;

    // A conflict over the block, both values retained.
    let conflict = applied(&ack).conflict.as_ref().expect("a conflict");
    assert_eq!(conflict.block_ids, vec![blocks[0].as_bytes().to_vec()]);

    // "today" is in the version that landed and keeps its anchor.
    assert_eq!(
        anchor_of(&w, today).await,
        (Some(note.as_uuid()), Some(blocks[0]), Some("today".into()))
    );
    // "eggs" is not, and its relation survives without an anchor.
    assert_eq!(w.relation_lifecycle(eggs).await.as_deref(), Some("active"));
    assert_eq!(
        anchor_of(&w, eggs).await,
        (Some(note.as_uuid()), None, None)
    );
}
