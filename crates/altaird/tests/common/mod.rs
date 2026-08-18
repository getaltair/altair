//! Shared setup for the write path's tests.
//!
//! Everything here is scaffolding. Nothing in it asserts anything, so a change
//! that makes a test pass has to be a change to the instance rather than to
//! this file.

#![allow(dead_code)]

use std::sync::Arc;

use altair_proto::v1;
use altaird::auth::Member;
use altaird::objects::FilesystemObjectStore;
use altaird::store::ids::EntityId;
use altaird::testing::TestDb;
use altaird::write::WritePath;
use chrono::{DateTime, Utc};
use sqlx::Row;
use tempfile::TempDir;
use uuid::Uuid;

/// A household with two members, and a write path over it.
pub struct World {
    pub db: Arc<TestDb>,
    pub household: Uuid,
    pub one: Member,
    pub two: Member,
    pub write: WritePath,
    /// Held for its lifetime: the object store's root, removed when this
    /// drops. Nothing reads it directly — `write.objects()` is the interface.
    pub object_root: TempDir,
}

impl World {
    pub async fn new() -> Self {
        let db = TestDb::new().await;
        let household = Uuid::new_v4();
        sqlx::query("INSERT INTO household (id, name, created_at) VALUES ($1, $2, now())")
            .bind(household)
            .bind("test")
            .execute(&db.pool)
            .await
            .expect("household");

        let one = seed_member(&db.pool, household, "one").await;
        let two = seed_member(&db.pool, household, "two").await;
        let object_root = TempDir::new().expect("temp object root");
        let store = FilesystemObjectStore::open(object_root.path())
            .await
            .expect("open the object store");
        let write = WritePath::new(db.pool.clone(), Arc::new(store));
        Self {
            db,
            household,
            one,
            two,
            write,
            object_root,
        }
    }

    /// Submit one intent and take the single acknowledgement back.
    pub async fn submit(&self, member: &Member, action: v1::intent::Action) -> v1::Acknowledgement {
        self.submit_as(member, Uuid::new_v4(), action).await
    }

    /// Submit under a stated intent identity, so a replay can reuse it.
    pub async fn submit_as(
        &self,
        member: &Member,
        intent: Uuid,
        action: v1::intent::Action,
    ) -> v1::Acknowledgement {
        let acks = self
            .write
            .submit(
                member,
                &[v1::Intent {
                    intent_id: intent.as_bytes().to_vec(),
                    action: Some(action),
                }],
            )
            .await
            .expect("the store was reachable");
        acks.into_iter().next().expect("one intent, one answer")
    }

    /// Create an entity of a type, with content, and return its identity.
    pub async fn create(
        &self,
        member: &Member,
        kind: v1::entity_content::Specific,
        content: v1::EntityContent,
    ) -> EntityId {
        let id = Uuid::new_v4();
        let ack = self
            .submit(
                member,
                create_entity(
                    id,
                    v1::EntityContent {
                        specific: Some(kind),
                        ..content
                    },
                ),
            )
            .await;
        assert!(
            matches!(ack.outcome, Some(v1::acknowledgement::Outcome::Applied(_))),
            "create was not applied: {:?}",
            ack.outcome
        );
        EntityId::from_uuid(id)
    }

    /// A note with a title and nothing else, which is the cheapest entity.
    pub async fn note(&self, member: &Member, title: &str) -> EntityId {
        self.create(
            member,
            v1::entity_content::Specific::Note(v1::NoteContent::default()),
            v1::EntityContent {
                title: Some(title.into()),
                ..Default::default()
            },
        )
        .await
    }

    pub async fn counter(&self, id: EntityId) -> i64 {
        sqlx::query("SELECT counter FROM entity WHERE id = $1")
            .bind(id.as_uuid())
            .fetch_one(&self.db.pool)
            .await
            .expect("entity")
            .try_get("counter")
            .expect("counter")
    }

    pub async fn title(&self, id: EntityId) -> Option<String> {
        sqlx::query("SELECT title FROM entity WHERE id = $1")
            .bind(id.as_uuid())
            .fetch_one(&self.db.pool)
            .await
            .expect("entity")
            .try_get("title")
            .expect("title")
    }

    pub async fn lifecycle(&self, id: EntityId) -> String {
        sqlx::query("SELECT lifecycle::text AS l FROM entity WHERE id = $1")
            .bind(id.as_uuid())
            .fetch_one(&self.db.pool)
            .await
            .expect("entity")
            .try_get("l")
            .expect("lifecycle")
    }

    pub async fn category_position(&self, id: EntityId) -> Option<i32> {
        sqlx::query("SELECT category_position AS p FROM entity WHERE id = $1")
            .bind(id.as_uuid())
            .fetch_one(&self.db.pool)
            .await
            .expect("entity")
            .try_get("p")
            .expect("position")
    }

    pub async fn relation_lifecycle(&self, id: Uuid) -> Option<String> {
        sqlx::query("SELECT lifecycle::text AS l FROM relation WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.db.pool)
            .await
            .expect("query")
            .map(|r| r.try_get("l").expect("lifecycle"))
    }

    /// The change sequence, in position order.
    pub async fn changes(&self) -> Vec<Change> {
        sqlx::query(
            "SELECT position, kind::text AS kind, entity_id, relation_id, \
             audience_before, audience_after FROM change ORDER BY position",
        )
        .fetch_all(&self.db.pool)
        .await
        .expect("changes")
        .iter()
        .map(|r| Change {
            position: r.try_get("position").unwrap(),
            kind: r.try_get("kind").unwrap(),
            entity: r.try_get("entity_id").unwrap(),
            relation: r.try_get("relation_id").unwrap(),
            audience_before: r.try_get("audience_before").unwrap(),
            audience_after: r.try_get("audience_after").unwrap(),
        })
        .collect()
    }

    pub async fn conflicts(&self, id: EntityId) -> Vec<Conflict> {
        sqlx::query(
            "SELECT field_name, mine_member_id, mine_value, theirs_member_id, theirs_value \
             FROM conflict WHERE entity_id = $1 ORDER BY field_name",
        )
        .bind(id.as_uuid())
        .fetch_all(&self.db.pool)
        .await
        .expect("conflicts")
        .iter()
        .map(|r| Conflict {
            field: r.try_get("field_name").unwrap(),
            mine_member: r.try_get("mine_member_id").unwrap(),
            mine: r.try_get("mine_value").unwrap(),
            theirs_member: r.try_get("theirs_member_id").unwrap(),
            theirs: r.try_get("theirs_value").unwrap(),
        })
        .collect()
    }

    /// How many rows a table holds for one entity, for the erasure test.
    pub async fn rows_for(&self, table: &str, column: &str, id: EntityId) -> i64 {
        let sql = format!("SELECT count(*) AS n FROM {table} WHERE {column} = $1");
        sqlx::query(sqlx::AssertSqlSafe(sql))
            .bind(id.as_uuid())
            .fetch_one(&self.db.pool)
            .await
            .expect("count")
            .try_get("n")
            .expect("n")
    }

    /// Declare a relation type. **Never one of the shipped set**: the point is
    /// that behaviour comes from the declaration.
    pub async fn declare_type(&self, label: &str, asymmetric: bool, quantified: bool) -> Uuid {
        let id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO relation_type (id, label, is_asymmetric, is_quantified) \
             VALUES ($1, $2, $3, $4)",
        )
        .bind(id)
        .bind(label)
        .bind(asymmetric)
        .bind(quantified)
        .execute(&self.db.pool)
        .await
        .expect("relation type");
        id
    }
}

pub struct Change {
    pub position: i64,
    pub kind: String,
    pub entity: Option<Uuid>,
    pub relation: Option<Uuid>,
    pub audience_before: Option<Vec<Uuid>>,
    pub audience_after: Option<Vec<Uuid>>,
}

pub struct Conflict {
    pub field: Option<String>,
    pub mine_member: Option<Uuid>,
    pub mine: Option<String>,
    pub theirs_member: Option<Uuid>,
    pub theirs: Option<String>,
}

async fn seed_member(pool: &sqlx::PgPool, household: Uuid, subject: &str) -> Member {
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO membership (id, household_id, subject, display_name, joined_at) \
         VALUES ($1, $2, $3, $3, now())",
    )
    .bind(id)
    .bind(household)
    .bind(subject)
    .execute(pool)
    .await
    .expect("membership");
    Member::for_test(id, household, subject, false)
}

// --- intent constructors -------------------------------------------------

pub fn create_entity(id: Uuid, content: v1::EntityContent) -> v1::intent::Action {
    v1::intent::Action::Create(v1::Create {
        subject: Some(v1::create::Subject::Entity(v1::CreateEntity {
            entity_id: id.as_bytes().to_vec(),
            created_at: None,
            capture_method: "test".into(),
            content: Some(content),
        })),
    })
}

pub fn edit_entity(id: EntityId, base: u64, content: v1::EntityContent) -> v1::intent::Action {
    v1::intent::Action::Edit(v1::Edit {
        subject: Some(v1::edit::Subject::Entity(v1::EditEntity {
            entity_id: id.as_uuid().as_bytes().to_vec(),
            base_counter: base,
            content: Some(content),
        })),
    })
}

pub fn create_relation(id: Uuid, content: v1::RelationContent) -> v1::intent::Action {
    v1::intent::Action::Create(v1::Create {
        subject: Some(v1::create::Subject::Relation(v1::CreateRelation {
            relation_id: id.as_bytes().to_vec(),
            content: Some(content),
        })),
    })
}

pub fn edit_relation(id: Uuid, content: v1::RelationContent) -> v1::intent::Action {
    v1::intent::Action::Edit(v1::Edit {
        subject: Some(v1::edit::Subject::Relation(v1::EditRelation {
            relation_id: id.as_bytes().to_vec(),
            content: Some(content),
        })),
    })
}

pub fn remove(entities: &[EntityId], relations: &[Uuid]) -> v1::intent::Action {
    v1::intent::Action::Remove(v1::Remove {
        entity_ids: entities
            .iter()
            .map(|e| e.as_uuid().as_bytes().to_vec())
            .collect(),
        relation_ids: relations.iter().map(|r| r.as_bytes().to_vec()).collect(),
    })
}

pub fn erase(entities: &[EntityId]) -> v1::intent::Action {
    v1::intent::Action::Erase(v1::Erase {
        entity_ids: entities
            .iter()
            .map(|e| e.as_uuid().as_bytes().to_vec())
            .collect(),
    })
}

pub fn restore(entity: EntityId, include_group: bool) -> v1::intent::Action {
    v1::intent::Action::Restore(v1::Restore {
        entity_id: entity.as_uuid().as_bytes().to_vec(),
        include_group,
    })
}

pub fn relation_between(from: EntityId, to: EntityId) -> v1::RelationContent {
    v1::RelationContent {
        from_entity_id: Some(from.as_uuid().as_bytes().to_vec()),
        to_entity_id: Some(to.as_uuid().as_bytes().to_vec()),
        ..Default::default()
    }
}

// --- acknowledgement readers ---------------------------------------------

pub fn applied(ack: &v1::Acknowledgement) -> &v1::Applied {
    match &ack.outcome {
        Some(v1::acknowledgement::Outcome::Applied(a)) => a,
        other => panic!("expected applied, got {other:?}"),
    }
}

pub fn refused(ack: &v1::Acknowledgement) -> &v1::Refused {
    match &ack.outcome {
        Some(v1::acknowledgement::Outcome::Refused(r)) => r,
        other => panic!("expected refused, got {other:?}"),
    }
}

pub fn recreated(ack: &v1::Acknowledgement) -> &v1::Recreated {
    match &ack.outcome {
        Some(v1::acknowledgement::Outcome::Recreated(r)) => r,
        other => panic!("expected recreated, got {other:?}"),
    }
}

pub fn is_not_available(ack: &v1::Acknowledgement) -> bool {
    match &ack.outcome {
        Some(v1::acknowledgement::Outcome::Refused(r)) => {
            r.reason == v1::RefusalReason::NotAvailable as i32
        }
        _ => false,
    }
}

pub fn relation_id(ack: &v1::Acknowledgement) -> Uuid {
    let a = applied(ack);
    let bytes: [u8; 16] = a.relation_ids[0].clone().try_into().expect("16 bytes");
    Uuid::from_bytes(bytes)
}

pub fn timestamp(at: DateTime<Utc>) -> altair_proto::prost_types::Timestamp {
    altair_proto::prost_types::Timestamp {
        seconds: at.timestamp(),
        nanos: at.timestamp_subsec_nanos() as i32,
    }
}
