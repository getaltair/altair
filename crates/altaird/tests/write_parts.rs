//! The part vocabulary round-trips.
//!
//! Migration one recorded the drift risk as gap (d): a conflicted part is named
//! by text in the store and by a field number on the wire, something has to
//! hold that mapping, and wherever it lives it can drift. This is what stops it
//! drifting silently.

use altaird::write::parts::{ALL_CONTENT_PARTS, ContentPart, Part, PartKey};
use uuid::Uuid;

#[test]
fn every_content_part_round_trips_through_its_field_number() {
    for part in ALL_CONTENT_PARTS {
        assert_eq!(
            ContentPart::from_field_number(part.field_number()),
            Some(*part),
            "{part:?} does not come back from its own field number"
        );
    }
}

#[test]
fn every_content_part_round_trips_through_its_stored_name() {
    for part in ALL_CONTENT_PARTS {
        assert_eq!(
            ContentPart::from_store_name(part.store_name()),
            Some(*part),
            "{part:?} does not come back from its own stored name"
        );
    }
}

#[test]
fn no_two_parts_share_a_number_or_a_name() {
    let mut numbers: Vec<u32> = ALL_CONTENT_PARTS.iter().map(|p| p.field_number()).collect();
    numbers.sort_unstable();
    let before = numbers.len();
    numbers.dedup();
    assert_eq!(before, numbers.len(), "two parts share a field number");

    let mut names: Vec<&str> = ALL_CONTENT_PARTS.iter().map(|p| p.store_name()).collect();
    names.sort_unstable();
    let before = names.len();
    names.dedup();
    assert_eq!(before, names.len(), "two parts share a stored name");
}

#[test]
fn a_number_naming_no_part_is_none() {
    // 6 is reserved: it was the entity-level arrangement key, and an order now
    // belongs to a container. 100 is `cleared` itself. 999 was never anything.
    for number in [0, 6, 100, 999] {
        assert_eq!(
            ContentPart::from_field_number(number),
            None,
            "field {number} is not a part and must not resolve to one"
        );
    }
}

#[test]
fn a_type_specific_part_cannot_be_read_as_a_content_part() {
    let specific = Part::Specific(1);
    let PartKey::Field(name) = specific.key() else {
        panic!("a type-specific part is keyed by name");
    };
    assert_eq!(ContentPart::from_store_name(&name), None);
    assert_eq!(Part::from_key(&PartKey::Field(name)), Some(specific));
}

#[test]
fn a_block_part_round_trips() {
    let id = Uuid::new_v4();
    let part = Part::Block(id);
    assert_eq!(Part::from_key(&part.key()), Some(part));
}

#[test]
fn a_name_this_build_does_not_know_is_not_guessed_at() {
    assert_eq!(
        Part::from_key(&PartKey::Field("something_a_later_build_wrote".into())),
        None,
        "an unknown part name must stay unknown; inventing one lets two \
         different parts collide"
    );
}

// --- type-specific parts -------------------------------------------------
//
// A type's fields are declared in `write/specific/`, per type, and the same
// drift is possible one level down: a field number is on the wire, a column
// name is in the store, and something has to hold the mapping. These walk every
// type's declaration, including the ones whose content no lane has filled in
// yet, so a mistyped number or a repeated column fails now rather than in
// whichever lane reaches it first.

use altaird::store::entity::EntityType;
use altaird::write::specific::{self, Held};

/// Every type there is. Written out rather than derived, so a type added to the
/// enum without a declaration fails to compile here.
const EVERY_TYPE: &[EntityType] = &[
    EntityType::Campaign,
    EntityType::Arc,
    EntityType::Quest,
    EntityType::Routine,
    EntityType::FocusSession,
    EntityType::CheckIn,
    EntityType::Note,
    EntityType::File,
    EntityType::Item,
    EntityType::Location,
    EntityType::ShoppingList,
    EntityType::Category,
];

#[test]
fn every_declared_field_round_trips_through_its_stored_key() {
    for kind in EVERY_TYPE {
        for field in specific::fields(*kind) {
            let part = Part::Specific(field.number);
            assert_eq!(
                Part::from_key(&part.key()),
                Some(part.clone()),
                "{kind:?} field {} does not come back from its own key",
                field.number
            );
            assert_eq!(
                specific::field(*kind, field.number),
                Some(*field),
                "{kind:?} field {} does not come back from its own number",
                field.number
            );
        }
    }
}

#[test]
fn no_two_fields_of_one_type_share_a_number_or_a_column() {
    for kind in EVERY_TYPE {
        let fields = specific::fields(*kind);

        let mut numbers: Vec<u32> = fields.iter().map(|f| f.number).collect();
        numbers.sort_unstable();
        let before = numbers.len();
        numbers.dedup();
        assert_eq!(before, numbers.len(), "{kind:?} declares a number twice");

        let mut columns: Vec<&str> = fields
            .iter()
            .filter_map(|f| match f.held {
                Held::Column(c) => Some(c),
                _ => None,
            })
            .collect();
        columns.sort_unstable();
        let before = columns.len();
        columns.dedup();
        assert_eq!(before, columns.len(), "{kind:?} declares a column twice");
    }
}

/// 100 is `cleared` itself in every one of these messages, and 0 is not a field
/// number at all. Either declared as a part would let a client clear the list
/// that says what it is clearing.
#[test]
fn no_type_declares_the_cleared_list_as_one_of_its_own_parts() {
    for kind in EVERY_TYPE {
        for field in specific::fields(*kind) {
            assert!(
                field.number != 0 && field.number != 100,
                "{kind:?} declares field {} as a part",
                field.number
            );
        }
        assert_eq!(specific::field(*kind, 100), None);
    }
}

/// A field held in a column needs a table for the column to be in.
#[test]
fn a_type_declaring_a_column_has_a_table_to_put_it_in() {
    for kind in EVERY_TYPE {
        let declares_a_column = specific::fields(*kind)
            .iter()
            .any(|f| matches!(f.held, Held::Column(_)));
        assert!(
            !declares_a_column || specific::side_table(*kind).is_some(),
            "{kind:?} declares a column and has no side table to put it in"
        );
    }
}

/// The three types the schema deliberately creates no table for address
/// nothing, and their messages reserve every number they will ever use.
#[test]
fn a_deferred_domain_declares_no_fields() {
    for kind in [
        EntityType::Routine,
        EntityType::FocusSession,
        EntityType::CheckIn,
    ] {
        assert!(specific::fields(kind).is_empty(), "{kind:?}");
        assert!(specific::side_table(kind).is_none(), "{kind:?}");
    }
}

/// The two spellings a stored key can have stay apart. A type-specific part is
/// prefixed so a field number can never be read as a content part's name, and
/// no content part's name can be read as a type-specific one.
#[test]
fn a_content_part_and_a_type_specific_part_cannot_be_confused() {
    for content in ALL_CONTENT_PARTS {
        let PartKey::Field(name) = Part::Content(*content).key() else {
            panic!("a content part is keyed by name");
        };
        assert!(
            !name.starts_with("specific."),
            "{content:?} is spelled like a type-specific part"
        );
    }
    for kind in EVERY_TYPE {
        for field in specific::fields(*kind) {
            let PartKey::Field(name) = Part::Specific(field.number).key() else {
                panic!("a type-specific part is keyed by name");
            };
            assert_eq!(ContentPart::from_store_name(&name), None);
        }
    }
}

/// **The known limit, recorded rather than missed.** Nothing here ties a
/// declared number to the tag the wire actually encodes. If a field's `tag` in
/// the proto changed, the reader would still read the same Rust field and would
/// still store it under the old number, and every test above would pass.
///
/// Closing it needs `prost::Message` in scope to encode a message and read the
/// key back, and `altair-proto` does not re-export `prost` — deliberately, it
/// generates and re-exports the contract and nothing else. The gap is narrow:
/// DR-004 makes field numbers permanent and a removed field is reserved rather
/// than reused, so a tag changing under a name that stayed is a violation of
/// the contract before it is a bug here.
#[test]
fn the_tie_to_the_wires_own_tags_is_a_known_gap() {
    // Nothing to assert. This test exists so the paragraph above is in the
    // suite rather than in a comment somebody deletes.
    assert_eq!(
        specific::field(EntityType::Campaign, 1).map(|f| f.held),
        Some(Held::Column("state"))
    );
}
