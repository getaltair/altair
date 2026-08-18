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

#[tokio::test]
async fn an_anchor_is_refused_for_a_reason_that_expires() {
    let w = World::new().await;
    let (a, b) = two_notes(&w).await;
    let ack = w
        .submit(
            &w.one,
            create_relation(
                Uuid::new_v4(),
                v1::RelationContent {
                    anchor: Some(v1::Anchor {
                        entity_id: a.as_uuid().as_bytes().to_vec(),
                        block_id: Uuid::new_v4().as_bytes().to_vec(),
                        phrase: String::new(),
                    }),
                    ..relation_between(a, b)
                },
            ),
        )
        .await;
    assert_eq!(refused(&ack).reason, v1::RefusalReason::Malformed as i32);
    assert!(
        !refused(&ack).detail.is_empty(),
        "silently discarding where somebody formed a connection is loss an \
         acknowledgement should never hide"
    );
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
