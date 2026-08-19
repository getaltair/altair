//! [`EntityRow`] to the wire, and nothing else.
//!
//! **Deliberately not in `entity.rs`, and that placement is load-bearing
//! rather than tidiness.** This file issues no SQL at all, which is what lets
//! it name `EntityContent`'s `audience_member_ids` field directly.
//! `entity.rs` issues SQL — `next_category_position`'s raw query, for one —
//! and `tests/one_predicate.rs` refuses any source outside
//! `store/audience.rs` that both names the audience column and issues SQL,
//! on the reasoning that a file able to run a query and able to spell the
//! column is exactly the shape a paraphrased predicate takes. Building this
//! conversion in `entity.rs` would have named that same wire field textually
//! and tripped the guard on a collision with an unrelated wire field name —
//! not a second predicate. Splitting the conversion out of the query surface
//! is what keeps the guard meaningful rather than papering over the
//! coincidence with an exemption. See `write::content`, which already relies
//! on the same distinction to read this field off an incoming message.

use chrono::{DateTime, Utc};

use altair_proto::v1;

use super::entity::{EntityRow, LifecycleState};

impl EntityRow {
    /// The shared fields, in the shape the wire's `Entity` carries them.
    ///
    /// **Type-specific content and blocks are not here.** Both need a join
    /// this row does not carry — the type's own side table, and `block` —
    /// and nothing on the read path needs them yet: Wave 3.1's literal arm
    /// and Wave 3.2's change stream both need no more than the shared fields
    /// to do their own job. A caller that needs `content.specific` or
    /// `blocks` fills them in afterwards; this conversion leaves both empty
    /// rather than guessing at a join its caller may not want paid for.
    #[must_use]
    pub fn into_wire(self) -> v1::Entity {
        let audience = self
            .audience
            .into_iter()
            .map(|m| m.as_uuid().as_bytes().to_vec())
            .collect();
        v1::Entity {
            entity_id: self.id.as_uuid().as_bytes().to_vec(),
            content: Some(v1::EntityContent {
                title: self.title,
                dates: Vec::new(),
                category_id: self.category_id.map(|c| c.as_uuid().as_bytes().to_vec()),
                category_position: self.category_position.map(|p| p as u32),
                assigned_member_ids: Vec::new(),
                audience_member_ids: audience,
                bulk: Some(self.bulk),
                cleared: Vec::new(),
                specific: None,
            }),
            author_member_id: self
                .author
                .map(|a| a.as_uuid().as_bytes().to_vec())
                .unwrap_or_default(),
            created_at: Some(timestamp(self.created_at)),
            updated_at: Some(timestamp(self.updated_at)),
            capture_method: self.capture_method,
            counter: self.counter as u64,
            lifecycle: match self.lifecycle {
                LifecycleState::Active => v1::LifecycleState::Active,
                LifecycleState::Deleted => v1::LifecycleState::Deleted,
                LifecycleState::Erased => v1::LifecycleState::Erased,
            } as i32,
            blocks: Vec::new(),
            conflicts: Vec::new(),
        }
    }
}

/// A wire timestamp from a stored one. The wire's own type, per
/// `altair-proto`'s module docs: "a timestamp on the wire is the wire's, and
/// it should come from the same place the messages holding it do."
fn timestamp(t: DateTime<Utc>) -> altair_proto::prost_types::Timestamp {
    altair_proto::prost_types::Timestamp {
        seconds: t.timestamp(),
        nanos: t.timestamp_subsec_nanos() as i32,
    }
}
