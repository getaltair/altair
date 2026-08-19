//! [`EntityRow`] to the wire, and nothing else.
//!
//! Split out of `entity.rs` on purpose and not merely by taste: this file
//! reaches no database, so it may name the wire's `audience_member_ids`
//! field — the same word `entity.rs`'s own audience column uses, by design —
//! without `tests/one_predicate.rs` mistaking a transcription for a second
//! predicate. See that test's own `issues_sql` doc for the distinction it
//! draws, and `write::content`, which already relies on the same one to read
//! that field off an incoming message.

use chrono::{DateTime, Utc};

use altair_proto::v1;

use super::entity::{EntityRow, LifecycleState};

fn timestamp(t: DateTime<Utc>) -> altair_proto::prost_types::Timestamp {
    altair_proto::prost_types::Timestamp {
        seconds: t.timestamp(),
        nanos: t.timestamp_subsec_nanos() as i32,
    }
}

impl EntityRow {
    /// The shared model, in the shape the wire states it.
    ///
    /// **Type-specific content and blocks are not here.** Both need a join
    /// this row does not carry — the type's own side table, and `block` — and
    /// nothing on the read path needs them yet: Wave 3.2's change stream and
    /// Wave 3.1's literal arm both need no more than the shared fields to do
    /// their own job. A caller that needs `content.specific` or `blocks`
    /// fills them in afterwards; this conversion leaves both empty rather
    /// than guessing at a join its caller may not want paid for.
    #[must_use]
    pub fn into_wire(self) -> v1::Entity {
        v1::Entity {
            entity_id: self.id.as_uuid().as_bytes().to_vec(),
            content: Some(v1::EntityContent {
                title: self.title,
                dates: Vec::new(),
                category_id: self.category_id.map(|id| id.as_uuid().as_bytes().to_vec()),
                category_position: self.category_position.map(|p| p as u32),
                assigned_member_ids: Vec::new(),
                audience_member_ids: self
                    .audience
                    .iter()
                    .map(|m| m.as_uuid().as_bytes().to_vec())
                    .collect(),
                bulk: Some(self.bulk),
                cleared: Vec::new(),
                specific: None,
            }),
            author_member_id: self
                .author
                .map(|m| m.as_uuid().as_bytes().to_vec())
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
