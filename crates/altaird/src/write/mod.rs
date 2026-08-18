//! The write path: the only way anything enters the store.
//!
//! # One transaction per intent
//!
//! **A bulk capture is explicitly not one unit.** Two hundred files where the
//! hundredth fails leaves ninety-nine captured, and the wire has no shape for
//! all-or-nothing to be expressed in — a submission is always a list and the
//! answer is always a list of the same length. So each intent gets its own
//! transaction, and this is forced rather than chosen. It is worth saying out
//! loud because the reflex, on seeing a batch, is to batch.
//!
//! What is atomic is one intent: the write it makes, the entries it puts in the
//! change sequence, and the intent row holding the acknowledgement all commit
//! together or none of them do.
//!
//! # A refusal is committed too
//!
//! An intent that is refused still writes its intent row, because the
//! acknowledgement is held and a replay has to get the same answer. A refusal
//! discovered partway through applying therefore rolls the transaction back and
//! opens a second one holding nothing but the refusal. That is the only place
//! two transactions are used for one intent, and the second writes no entity
//! content at all.
//!
//! # What is a fault and what is an answer
//!
//! A refusal is an answer: it is committed, it is replayed, and the submission
//! it was part of still succeeds. **A store fault is not.** Nothing is
//! acknowledged, the caller's outbox holds, and the caller waits — which is the
//! same wait an unreachable instance produces, because to the person they are
//! the same thing.

pub mod body;
pub mod changes;
pub mod content;
pub mod entity;
pub mod intent;
pub mod outcome;
pub mod parts;
pub mod provenance;
pub mod relation;
pub mod specific;

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use altair_proto::v1;

use crate::auth::Member;
use crate::store::begin_write;
use crate::store::ids::{EntityId, MemberId};

use content::{entity_type, identifier, instant, parts_written};
use entity::{Ctx, Failed, Refusal};
use outcome::{Outcome, RefusalReason};

/// The instance failing, never a caller's intent failing.
///
/// Deliberately narrow. Everything a caller can do wrong is an [`Outcome`].
#[derive(Debug)]
pub struct Fault(pub sqlx::Error);

impl std::fmt::Display for Fault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the structured store was unavailable: {}", self.0)
    }
}

impl std::error::Error for Fault {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

/// The write path.
#[derive(Clone)]
pub struct WritePath {
    pool: PgPool,
}

impl WritePath {
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Apply a submission, one intent at a time.
    ///
    /// **Never all or nothing.** The answer has one acknowledgement per intent,
    /// in the order submitted, and an intent that was refused does not affect
    /// the one after it.
    ///
    /// # Errors
    ///
    /// Only [`Fault`], and only where the store itself could not be reached or
    /// written. No property of an intent produces one.
    pub async fn submit(
        &self,
        member: &Member,
        intents: &[v1::Intent],
    ) -> Result<Vec<v1::Acknowledgement>, Fault> {
        let mut acknowledgements = Vec::with_capacity(intents.len());
        for intent in intents {
            acknowledgements.push(self.one(member, intent).await?);
        }
        Ok(acknowledgements)
    }

    async fn one(
        &self,
        member: &Member,
        intent: &v1::Intent,
    ) -> Result<v1::Acknowledgement, Fault> {
        // The identity is the client's, and a malformed one cannot be held
        // against a replay, so it is answered without a row. There is nothing
        // to be idempotent about: a resubmission of the same bytes gets the
        // same answer by computing it again.
        let Ok(id) = identifier(&intent.intent_id) else {
            return Ok(acknowledge(
                &intent.intent_id,
                &Outcome::malformed("an intent identity is 16 bytes"),
            ));
        };

        let outcome = self.apply(member, id, intent).await?;
        Ok(acknowledge(&intent.intent_id, &outcome))
    }

    async fn apply(
        &self,
        member: &Member,
        id: Uuid,
        intent: &v1::Intent,
    ) -> Result<Outcome, Fault> {
        let at = Utc::now();
        // The write path's requester. `assert_participating` is what the
        // audience predicate trusts, and this is a legitimate caller of it:
        // token validation resolved the subject to a membership and filtered
        // departed members before a `Member` existed.
        let requester = MemberId::assert_participating(member.membership_id());

        let mut tx = begin_write(&self.pool).await.map_err(Fault)?;

        match intent::held(&mut tx, id, member.membership_id())
            .await
            .map_err(Fault)?
        {
            intent::Held::Mine(outcome) => {
                // Nothing was written, so there is nothing to commit. Rolling
                // back is the honest ending, and it is what makes replay free.
                tx.rollback().await.map_err(Fault)?;
                return Ok(outcome);
            }
            intent::Held::SomebodyElses => {
                tx.rollback().await.map_err(Fault)?;
                return Ok(Outcome::not_available());
            }
            intent::Held::Absent => {}
        }

        // Before anything is written. See `changes` for why this is first.
        changes::hold_the_sequence(&mut tx).await.map_err(Fault)?;

        let mut ctx = Ctx {
            tx: &mut tx,
            member: requester,
            at,
        };
        let applied = act(&mut ctx, intent).await;

        let outcome = match applied {
            Ok(outcome) => outcome,
            Err(Failed::Store(e)) => {
                tx.rollback().await.ok();
                return Err(Fault(e));
            }
            Err(Failed::Refused(refusal)) => {
                // Whatever this intent had already written is discarded, and a
                // second transaction holds nothing but the refusal.
                tx.rollback().await.map_err(Fault)?;
                let outcome = match refusal {
                    Refusal::NotAvailable => Outcome::not_available(),
                    Refusal::Malformed(detail) => Outcome::malformed(detail),
                };
                let mut tx = begin_write(&self.pool).await.map_err(Fault)?;
                let held = intent::hold(&mut tx, id, member.membership_id(), at, &outcome)
                    .await
                    .map_err(Fault)?;
                if !held {
                    tx.rollback().await.map_err(Fault)?;
                    return self.replay(id, member).await;
                }
                tx.commit().await.map_err(Fault)?;
                return Ok(outcome);
            }
        };

        // Somebody else acknowledged this intent while this transaction was
        // working. Their answer is the answer, and everything written here is
        // discarded — which is what keeps "a second effect" impossible rather
        // than merely unlikely.
        if !intent::hold(&mut tx, id, member.membership_id(), at, &outcome)
            .await
            .map_err(Fault)?
        {
            tx.rollback().await.map_err(Fault)?;
            return self.replay(id, member).await;
        }
        tx.commit().await.map_err(Fault)?;
        Ok(outcome)
    }

    /// The acknowledgement already issued for an intent, read on its own.
    ///
    /// Reached only when a concurrent submission of the same identity won the
    /// race. It cannot find nothing: the insert that lost only lost because a
    /// committed row is there.
    async fn replay(&self, id: Uuid, member: &Member) -> Result<Outcome, Fault> {
        let mut tx = begin_write(&self.pool).await.map_err(Fault)?;
        let held = intent::held(&mut tx, id, member.membership_id())
            .await
            .map_err(Fault)?;
        tx.rollback().await.map_err(Fault)?;
        Ok(match held {
            intent::Held::Mine(outcome) => outcome,
            // Another member holds it, which is the same nothing as always.
            intent::Held::SomebodyElses | intent::Held::Absent => Outcome::not_available(),
        })
    }
}

/// Dispatch one intent's action.
async fn act(ctx: &mut Ctx<'_>, intent: &v1::Intent) -> Result<Outcome, Failed> {
    use v1::intent::Action;

    match intent.action.as_ref() {
        Some(Action::Create(create)) => match create.subject.as_ref() {
            Some(v1::create::Subject::Entity(e)) => {
                let content = e.content.as_ref().ok_or_else(|| {
                    Refusal::Malformed("a create carries the content that says what it is".into())
                })?;
                let kind = entity_type(content)?;
                let written = parts_written(content)?;
                let id = EntityId::from_uuid(identifier(&e.entity_id)?);
                // Absent is ordinary and means "the instance's clock". Present
                // and unreadable is a malformed message, and it goes through the
                // same parser a labelled date does — see `content::instant` for
                // why the two used to disagree.
                let created_at = e.created_at.as_ref().map(instant).transpose()?;
                entity::create(ctx, id, created_at, &e.capture_method, written, kind).await
            }
            Some(v1::create::Subject::Relation(r)) => {
                let content = r.content.as_ref().ok_or_else(|| {
                    Refusal::Malformed("a relation create carries the relation".into())
                })?;
                let id = identifier(&r.relation_id)?;
                let id = relation::create(ctx, id, content).await?;
                Ok(Outcome::Applied {
                    entities: Vec::new(),
                    relations: vec![id],
                    counter: None,
                    conflict: None,
                })
            }
            None => Err(Refusal::Malformed("a create names no subject".into()).into()),
        },

        Some(Action::Edit(edit)) => match edit.subject.as_ref() {
            Some(v1::edit::Subject::Entity(e)) => {
                let content = e.content.as_ref().ok_or_else(|| {
                    Refusal::Malformed("an edit carries the content it is writing".into())
                })?;
                // The type is the tag, and an edit need not restate it. Where
                // it does, it must agree.
                let stated = content
                    .specific
                    .as_ref()
                    .map(|_| entity_type(content))
                    .transpose()?;
                let written = parts_written(content)?;
                let id = EntityId::from_uuid(identifier(&e.entity_id)?);
                // Clamping was worse than it looks. A base counter larger than
                // any counter the store can hold reads as *current* against
                // every entity, so a garbled value silently skipped conflict
                // detection entirely — the one mechanism this item exists to
                // build — rather than being refused as the unreadable message
                // it is.
                //
                // **Zero is refused for the same reason, from the other end.**
                // The first counter an entity has is 1, issued by its own
                // create, so zero is not a counter this instance could have
                // issued either — and it is the value the wire produces when a
                // client omits the field, because `base_counter` is a proto3
                // `uint64` whose default is indistinguishable from unset.
                //
                // Accepting it was not harmless. A create records the parts it
                // applied without comparing them, because on a create there is
                // no prior value to compare against; so an entity created with
                // its title explicitly cleared carries provenance for a title
                // that never changed. An edit arriving with base zero reads
                // `0 < 1`, asks what moved since, finds that record, and retains
                // a conflict over a part only the second write ever touched.
                // The client that hits this is one that forgot a field, not one
                // doing anything exotic.
                if e.base_counter == 0 {
                    return Err(Refusal::Malformed(
                        "a base counter is a counter this instance could have issued,                          and the first one is 1"
                            .into(),
                    )
                    .into());
                }
                let base = i64::try_from(e.base_counter).map_err(|_| {
                    Refusal::Malformed(
                        "a base counter is a counter this instance could have issued".into(),
                    )
                })?;
                entity::edit(ctx, id, base, written, stated).await
            }
            Some(v1::edit::Subject::Relation(r)) => {
                let content = r.content.as_ref().ok_or_else(|| {
                    Refusal::Malformed("a relation edit carries the relation".into())
                })?;
                let id = identifier(&r.relation_id)?;
                let id = relation::edit(ctx, id, content).await?;
                Ok(Outcome::Applied {
                    entities: Vec::new(),
                    relations: vec![id],
                    counter: None,
                    conflict: None,
                })
            }
            None => Err(Refusal::Malformed("an edit names no subject".into()).into()),
        },

        Some(Action::Remove(remove)) => {
            // Everything named in one Remove is one act, and that grouping is
            // retained so that restoring any one of them can bring back the
            // rest — including the connections the same act removed.
            let group = Uuid::new_v4();
            let mut entities = Vec::new();
            for b in &remove.entity_ids {
                entities.push(EntityId::from_uuid(identifier(b)?));
            }
            let mut relations = Vec::new();
            for b in &remove.relation_ids {
                relations.push(identifier(b)?);
            }
            let entities = entity::remove(ctx, &entities, group).await?;
            let relations = relation::remove(ctx, &relations, group).await?;
            Ok(Outcome::Applied {
                entities,
                relations,
                counter: None,
                conflict: None,
            })
        }

        Some(Action::Erase(erase)) => {
            let mut ids = Vec::new();
            for b in &erase.entity_ids {
                ids.push(EntityId::from_uuid(identifier(b)?));
            }
            let (entities, relations) = entity::erase(ctx, &ids).await?;
            Ok(Outcome::Applied {
                entities,
                relations,
                counter: None,
                conflict: None,
            })
        }

        Some(Action::Restore(restore)) => {
            let id = EntityId::from_uuid(identifier(&restore.entity_id)?);
            let (entities, group) = entity::restore(ctx, id, restore.include_group).await?;
            let relations = match group {
                Some(group) => relation::restore_group(ctx, group).await?,
                None => Vec::new(),
            };
            Ok(Outcome::Applied {
                entities,
                relations,
                counter: None,
                conflict: None,
            })
        }

        None => Err(Refusal::Malformed("an intent names no action".into()).into()),
    }
}

/// The acknowledgement, in the wire's shape.
fn acknowledge(intent_id: &[u8], outcome: &Outcome) -> v1::Acknowledgement {
    use v1::acknowledgement::Outcome as Wire;

    let outcome = match outcome {
        Outcome::Applied {
            entities,
            relations,
            counter,
            conflict,
        } => Wire::Applied(v1::Applied {
            entity_ids: entities
                .iter()
                .map(|e| e.as_uuid().as_bytes().to_vec())
                .collect(),
            relation_ids: relations.iter().map(|r| r.as_bytes().to_vec()).collect(),
            counter: counter.unwrap_or(0).try_into().unwrap_or(0),
            conflict: conflict.as_ref().map(|c| v1::ConflictRetained {
                content_fields: c.content_fields.clone(),
                specific_fields: c.specific_fields.clone(),
                block_ids: c.block_ids.iter().map(|b| b.as_bytes().to_vec()).collect(),
            }),
        }),
        Outcome::Refused { reason, detail } => Wire::Refused(v1::Refused {
            reason: match reason {
                RefusalReason::NotAvailable => v1::RefusalReason::NotAvailable as i32,
                RefusalReason::Malformed => v1::RefusalReason::Malformed as i32,
            },
            detail: detail.clone(),
        }),
        Outcome::Recreated {
            original,
            new,
            counter,
        } => Wire::Recreated(v1::Recreated {
            original_entity_id: original.as_uuid().as_bytes().to_vec(),
            new_entity_id: new.as_uuid().as_bytes().to_vec(),
            counter: (*counter).try_into().unwrap_or(0),
        }),
    };

    v1::Acknowledgement {
        intent_id: intent_id.to_vec(),
        outcome: Some(outcome),
    }
}
