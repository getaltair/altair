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
