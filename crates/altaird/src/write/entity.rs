//! The five verbs, on an entity.
//!
//! Everything here runs inside one transaction that has already taken the
//! change sequence's lock; see [`super::changes`] for why that ordering
//! matters.
//!
//! # What this item writes, and what it leaves to 2.2
//!
//! The shared model — title, dates, category and the position inside it,
//! assignments, audience, bulk — is written here. **Type-specific content is
//! not.** A create makes the side-table row with its defaults so nothing is
//! ever half-formed, and the fields inside `content.specific` are left for
//! Wave 2.2, which owns bodies, anchors, the ladder, and cycle prevention in
//! nested containers.
//!
//! One type is still refused for a reason that expires, which is the honest
//! answer rather than an omission:
//!
//! - **A routine, a focus session, or a check-in**, until each domain is
//!   designed. The schema deliberately creates no table for them, so there is
//!   nothing to put a row in.
//!
//! **A file is no longer one of them.** Wave 2.3 lands `PutBody`, and a file
//! create now checks the named body exists — bytes before the record,
//! always — before writing the row. `content.specific` stays otherwise
//! unread here; the file's `body_id` is the one field read at creation, by
//! [`super::content::file_reference`], because the schema cannot create the
//! row without it. See that function's doc for why this is a narrow
//! exception rather than a reversal of the rule above it.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use sqlx::Row;
use uuid::Uuid;

use crate::objects::{BodyId, ObjectStore};
use crate::store::audience::AUDIENCE_COLUMN;
use crate::store::entity::{EntityRow, EntityType, LifecycleState, available_for_write};
use crate::store::ids::{EntityId, MemberId, MemberRef};
use crate::store::{WriteScope, WriteTx};

use super::changes::{self, EntityChange};
use super::content::{Date, FileReference, Malformed, PartWrite};
use super::outcome::{ConflictParts, Outcome};
use super::provenance;

/// One intent's worth of context: the transaction, who is writing, when, and
/// the object store a file create has to check against.
pub struct Ctx<'a> {
    pub tx: &'a mut WriteTx,
    pub member: MemberId,
    pub at: DateTime<Utc>,
    pub store: Arc<dyn ObjectStore>,
}

impl Ctx<'_> {
    fn author(&self) -> MemberRef {
        MemberRef::from_uuid(self.member.as_uuid())
    }
}

/// A refusal raised from inside the apply path, which the spine turns into an
/// acknowledgement after rolling the transaction back.
pub enum Refusal {
    NotAvailable,
    Malformed(String),
}

impl From<Malformed> for Refusal {
    fn from(m: Malformed) -> Self {
        Self::Malformed(m.0)
    }
}

/// Either a refusal or a store fault. The two are kept apart all the way out:
/// a refusal is an answer, a fault is the instance failing and produces no
/// acknowledgement at all.
///
/// **Two fault variants**, because DR-003 keeps the structured store and the
/// object store as separate failures with separate causes. Both still mean
/// the same thing to the caller — nothing was acknowledged, the outbox holds.
pub enum Failed {
    Refused(Refusal),
    Store(sqlx::Error),
    Objects(crate::objects::Error),
}

impl From<sqlx::Error> for Failed {
    fn from(e: sqlx::Error) -> Self {
        Self::Store(e)
    }
}

impl From<crate::objects::Error> for Failed {
    fn from(e: crate::objects::Error) -> Self {
        Self::Objects(e)
    }
}

impl From<Refusal> for Failed {
    fn from(r: Refusal) -> Self {
        Self::Refused(r)
    }
}

impl From<Malformed> for Failed {
    fn from(m: Malformed) -> Self {
        Self::Refused(Refusal::Malformed(m.0))
    }
}

pub type Applied<T> = Result<T, Failed>;

/// Every member named must answer to a real membership.
///
/// **A foreign key cannot reach inside an array**, which is what the schema
/// says about the audience column and is the reason this check exists at all.
/// It covers assignments too, where a foreign key does exist, so that a bad
/// identifier is a refusal rather than a constraint violation that takes the
/// whole transaction down with a message nobody can act on.
pub async fn memberships_exist(tx: &mut WriteTx, ids: &[Uuid]) -> Applied<bool> {
    if ids.is_empty() {
        return Ok(true);
    }
    let mut wanted: Vec<Uuid> = ids.to_vec();
    wanted.sort_unstable();
    wanted.dedup();
    let row = sqlx::query("SELECT count(*) AS n FROM membership WHERE id = ANY($1)")
        .bind(&wanted)
        .fetch_one(tx.conn())
        .await?;
    Ok(row.try_get::<i64, _>("n")? == wanted.len() as i64)
}

/// The value a part currently holds, rendered by the same code the arriving
/// value goes through.
async fn current(ctx: &mut Ctx<'_>, row: &EntityRow, part: &PartWrite) -> Applied<PartWrite> {
    Ok(match part {
        PartWrite::Title(_) => PartWrite::Title(row.title.clone()),
        PartWrite::Dates(_) => {
            let rows = sqlx::query(
                "SELECT label, occurs_at, bring_forward FROM entity_date \
                 WHERE entity_id = $1 ORDER BY ordinal",
            )
            .bind(row.id.as_uuid())
            .fetch_all(ctx.tx.conn())
            .await?;
            let mut dates = Vec::with_capacity(rows.len());
            for r in &rows {
                dates.push(Date {
                    label: r.try_get("label")?,
                    occurs_at: r.try_get("occurs_at")?,
                    bring_forward: r.try_get("bring_forward")?,
                });
            }
            PartWrite::Dates(dates)
        }
        PartWrite::Category(_) => PartWrite::Category(row.category_id),
        PartWrite::CategoryPosition(_) => PartWrite::CategoryPosition(row.category_position),
        PartWrite::Assignments(_) => {
            let rows = sqlx::query("SELECT member_id FROM entity_assignment WHERE entity_id = $1")
                .bind(row.id.as_uuid())
                .fetch_all(ctx.tx.conn())
                .await?;
            let mut ids = Vec::with_capacity(rows.len());
            for r in &rows {
                ids.push(r.try_get("member_id")?);
            }
            PartWrite::Assignments(ids)
        }
        PartWrite::Audience(_) => PartWrite::Audience(
            row.audience
                .iter()
                .copied()
                .map(MemberRef::as_uuid)
                .collect(),
        ),
        PartWrite::Bulk(_) => PartWrite::Bulk(Some(row.bulk)),
    })
}

/// Apply one part to the store. Assumes it has already been validated.
///
/// `placement` is the position the instance assigned as a consequence, which is
/// only ever set alongside a category. **The two move in one statement**,
/// because the store's own check says a category and a position are either both
/// there or both absent: writing them in sequence puts the row through a state
/// the schema refuses, whichever order the sequence is in.
async fn apply_part(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    part: &PartWrite,
    placement: Option<&PartWrite>,
) -> Applied<()> {
    match part {
        PartWrite::Title(v) => {
            // search_text is maintained by the write path from the title and
            // whatever the type contributes, because those live in side tables
            // and a generated column cannot reach them. Only the title
            // contributes here; the type's share arrives with 2.2.
            sqlx::query("UPDATE entity SET title = $2, search_text = $3 WHERE id = $1")
                .bind(entity.as_uuid())
                .bind(v.as_deref())
                .bind(v.as_deref().unwrap_or(""))
                .execute(ctx.tx.conn())
                .await?;
        }
        PartWrite::Dates(dates) => {
            sqlx::query("DELETE FROM entity_date WHERE entity_id = $1")
                .bind(entity.as_uuid())
                .execute(ctx.tx.conn())
                .await?;
            for (ordinal, d) in dates.iter().enumerate() {
                sqlx::query(
                    "INSERT INTO entity_date (entity_id, ordinal, label, occurs_at, bring_forward) \
                     VALUES ($1, $2, $3, $4, $5)",
                )
                .bind(entity.as_uuid())
                .bind(i32::try_from(ordinal).unwrap_or(i32::MAX))
                .bind(&d.label)
                .bind(d.occurs_at)
                .bind(d.bring_forward)
                .execute(ctx.tx.conn())
                .await?;
            }
        }
        PartWrite::Category(v) => {
            let position = match placement {
                Some(PartWrite::CategoryPosition(p)) => *p,
                _ => None,
            };
            sqlx::query("UPDATE entity SET category_id = $2, category_position = $3 WHERE id = $1")
                .bind(entity.as_uuid())
                .bind(v.map(EntityId::as_uuid))
                .bind(position)
                .execute(ctx.tx.conn())
                .await?;
        }
        PartWrite::CategoryPosition(v) => {
            sqlx::query("UPDATE entity SET category_position = $2 WHERE id = $1")
                .bind(entity.as_uuid())
                .bind(*v)
                .execute(ctx.tx.conn())
                .await?;
        }
        PartWrite::Assignments(ids) => {
            sqlx::query("DELETE FROM entity_assignment WHERE entity_id = $1")
                .bind(entity.as_uuid())
                .execute(ctx.tx.conn())
                .await?;
            for id in ids {
                sqlx::query(
                    "INSERT INTO entity_assignment (entity_id, member_id) VALUES ($1, $2) \
                     ON CONFLICT DO NOTHING",
                )
                .bind(entity.as_uuid())
                .bind(id)
                .execute(ctx.tx.conn())
                .await?;
            }
        }
        PartWrite::Audience(ids) => {
            let sql = format!("UPDATE entity SET {AUDIENCE_COLUMN} = $2 WHERE id = $1");
            sqlx::query(sqlx::AssertSqlSafe(sql))
                .bind(entity.as_uuid())
                .bind(ids)
                .execute(ctx.tx.conn())
                .await?;
        }
        PartWrite::Bulk(v) => {
            sqlx::query("UPDATE entity SET bulk = $2 WHERE id = $1")
                .bind(entity.as_uuid())
                .bind(v.unwrap_or(false))
                .execute(ctx.tx.conn())
                .await?;
        }
    }
    Ok(())
}

/// Checks a part owes before it is applied, and the placement the instance
/// makes rather than the client.
///
/// Returns any extra part the instance moved as a consequence — a position,
/// when a category changed.
async fn validate_and_place(
    ctx: &mut Ctx<'_>,
    entering: Option<EntityId>,
    part: &PartWrite,
) -> Applied<Option<PartWrite>> {
    match part {
        PartWrite::Audience(ids) | PartWrite::Assignments(ids) => {
            if !memberships_exist(ctx.tx, ids).await? {
                return Err(Refusal::NotAvailable.into());
            }
            Ok(None)
        }
        PartWrite::CategoryPosition(_) => Err(Refusal::Malformed(
            "position is assigned by the instance, which appends on entry to a container; \
             explicit reordering is not served yet"
                .into(),
        )
        .into()),
        PartWrite::Category(Some(category)) => {
            // The container has to be one this member can see, and it has to be
            // a category. Both refusals are the same nothing.
            let found = available_for_write(ctx.tx, ctx.member, *category, WriteScope::Active)
                .await?
                .ok_or(Refusal::NotAvailable)?;
            if found.entity_type != EntityType::Category {
                return Err(Refusal::NotAvailable.into());
            }
            // Entering a container places the entity at the end. This holds
            // whether the entity is newly created, moved from another
            // container, or restored, and it is safe as a read-then-write only
            // because the sequence row is already held.
            let entering_this = entering != Some(*category);
            if entering_this {
                let next = crate::store::entity::next_category_position(ctx.tx, *category).await?;
                return Ok(Some(PartWrite::CategoryPosition(Some(next))));
            }
            Ok(None)
        }
        // Leaving a container forgets the position. Nothing is carried and
        // nothing needs repair.
        PartWrite::Category(None) => Ok(Some(PartWrite::CategoryPosition(None))),
        _ => Ok(None),
    }
}

/// The side-table row a type carries beyond the shared model, with its
/// defaults.
///
/// Note and shopping list have no table: a body is its blocks in order and
/// there is nothing else, which is a result rather than an omission.
async fn make_type_row(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    file: Option<FileReference>,
) -> Applied<()> {
    let table = match kind {
        EntityType::Campaign => "campaign",
        EntityType::Arc => "arc",
        EntityType::Quest => "quest",
        EntityType::Item => "item",
        EntityType::Location => "location",
        EntityType::Category => "category",
        EntityType::Note | EntityType::ShoppingList => return Ok(()),
        EntityType::File => return create_file_row(ctx, entity, file).await,
        EntityType::Routine | EntityType::FocusSession | EntityType::CheckIn => {
            return Err(Refusal::Malformed(
                "this type's content is not yet designed and the store has no table for it".into(),
            )
            .into());
        }
    };
    let sql = format!("INSERT INTO {table} (entity_id) VALUES ($1)");
    sqlx::query(sqlx::AssertSqlSafe(sql))
        .bind(entity.as_uuid())
        .execute(ctx.tx.conn())
        .await?;
    Ok(())
}

/// **Bytes before the record, always.** The body a file create names has to
/// already be durable in the object store — confirmed here by asking for it
/// and reading its length, never its bytes — before this transaction can
/// insert a row that points at it. A kill between `PutBody` completing and
/// this transaction committing leaves the bytes as collectable garbage
/// (Wave 2.4's to sweep) and never a `file` row pointing at nothing.
async fn create_file_row(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    file: Option<FileReference>,
) -> Applied<()> {
    let file = file.ok_or_else(|| Refusal::Malformed("a file names no body".into()))?;

    let body = match ctx.store.get(BodyId::from_uuid(file.body_id)).await {
        Ok(body) => body,
        // Never uploaded, or already swept — both read as "this create does
        // not make sense" from the wire's perspective, not "something else
        // is wrong".
        Err(e) if e.is_no_such_body() => {
            return Err(Refusal::Malformed(
                "a file names a body that was never uploaded, or has already been swept".into(),
            )
            .into());
        }
        Err(e) => return Err(e.into()),
    };

    sqlx::query(
        "INSERT INTO file (entity_id, body_id, media_type, byte_size) VALUES ($1, $2, $3, $4)",
    )
    .bind(entity.as_uuid())
    .bind(file.body_id)
    .bind(file.media_type)
    .bind(i64::try_from(body.len).unwrap_or(i64::MAX))
    .execute(ctx.tx.conn())
    .await?;
    Ok(())
}

/// A category's creation default, which acts once and is never inherited.
async fn creation_default_audience(ctx: &mut Ctx<'_>, category: EntityId) -> Applied<Vec<Uuid>> {
    let row =
        sqlx::query("SELECT creation_default_audience AS a FROM category WHERE entity_id = $1")
            .bind(category.as_uuid())
            .fetch_optional(ctx.tx.conn())
            .await?;
    Ok(row
        .map(|r| r.try_get::<Vec<Uuid>, _>("a"))
        .transpose()?
        .unwrap_or_default())
}

/// Create an entity.
pub async fn create(
    ctx: &mut Ctx<'_>,
    id: EntityId,
    created_at: Option<DateTime<Utc>>,
    capture_method: &str,
    parts: Vec<PartWrite>,
    kind: EntityType,
    file: Option<FileReference>,
) -> Applied<Outcome> {
    // A create naming something that is already here is not a second entity.
    // The outbox's idempotence is carried by intent identity, but the substrate
    // states the rule about entities directly — the same entity submitted twice
    // produces one entity, not two — and a create arriving under a fresh intent
    // identity is exactly that case.
    if let Some(existing) =
        available_for_write(ctx.tx, ctx.member, id, WriteScope::AnyIncludingErased).await?
    {
        // A tombstone is not a free identity. Recreating under an erased id
        // would reuse an identity somebody erased, and anything that once
        // pointed at it would silently reattach.
        if existing.lifecycle == LifecycleState::Erased || existing.entity_type != kind {
            return Err(Refusal::NotAvailable.into());
        }
        return Ok(Outcome::applied_entity(id, existing.counter));
    }
    // Either it is not there, or it is there and invisible. The lookup above
    // cannot tell those apart — that is what the audience predicate is for —
    // so the insert does, and it does it without ever raising a store fault.
    //
    // `ON CONFLICT DO NOTHING` with the row count read is the whole mechanism.
    // A bare insert makes the invisible case a primary-key violation, which
    // reaches the caller as a store fault while an unused identifier reaches
    // them as a success, and a fault that appears only when something is there
    // is an oracle for whether something is there. This is the exact
    // disclosure the single refusal reason exists to prevent, arriving through
    // the error channel rather than through the answer.

    let created_at = created_at.unwrap_or(ctx.at);

    // Audience is private to the author unless the write says otherwise, or the
    // category it lands in states a creation default. The default acts once, at
    // creation, and an explicit audience outranks it.
    let stated_audience = parts.iter().any(|p| matches!(p, PartWrite::Audience(_)));
    let category = parts.iter().find_map(|p| match p {
        PartWrite::Category(Some(c)) => Some(*c),
        _ => None,
    });

    let sql = format!(
        "INSERT INTO entity \
         (id, type, title, author_member_id, created_at, updated_at, capture_method, \
          bulk, {AUDIENCE_COLUMN}, lifecycle, counter, search_text) \
         VALUES ($1, $2::entity_type, NULL, $3, $4, $5, $6, false, '{{}}', 'active', 1, '') \
         ON CONFLICT (id) DO NOTHING"
    );
    let inserted = sqlx::query(sqlx::AssertSqlSafe(sql))
        .bind(id.as_uuid())
        .bind(type_name(kind))
        .bind(ctx.member.as_uuid())
        .bind(created_at)
        .bind(ctx.at)
        .bind(capture_method)
        .execute(ctx.tx.conn())
        .await?;
    if inserted.rows_affected() == 0 {
        return Err(Refusal::NotAvailable.into());
    }

    make_type_row(ctx, id, kind, file).await?;

    let mut written = Vec::new();
    for part in &parts {
        let placement = validate_and_place(ctx, None, part).await?;
        apply_part(ctx, id, part, placement.as_ref()).await?;
        written.push(part.part());
        if let Some(placement) = &placement {
            written.push(placement.part());
        }
    }

    if !stated_audience && let Some(category) = category {
        let default = creation_default_audience(ctx, category).await?;
        if !default.is_empty() {
            if !memberships_exist(ctx.tx, &default).await? {
                return Err(Refusal::NotAvailable.into());
            }
            let part = PartWrite::Audience(default);
            apply_part(ctx, id, &part, None).await?;
            written.push(part.part());
        }
    }

    provenance::record(ctx.tx, id, &written, 1, ctx.member).await?;

    let after = read_audience(ctx, id).await?;
    changes::entity_written(
        ctx.tx,
        ctx.at,
        &EntityChange {
            entity: id,
            // Nothing was there, so there is no audience before. Null rather
            // than empty: empty means private to the author, which is a state
            // the entity was never in.
            audience_before: None,
            audience_after: Some(after),
            author_before: None,
            author_after: Some(ctx.author()),
            changed_blocks: Vec::new(),
        },
    )
    .await?;

    Ok(Outcome::applied_entity(id, 1))
}

async fn read_audience(ctx: &mut Ctx<'_>, id: EntityId) -> Applied<Vec<MemberRef>> {
    let row = available_for_write(ctx.tx, ctx.member, id, WriteScope::AnyIncludingErased).await?;
    Ok(row.map(|r| r.audience).unwrap_or_default())
}

fn type_name(kind: EntityType) -> &'static str {
    match kind {
        EntityType::Campaign => "campaign",
        EntityType::Arc => "arc",
        EntityType::Quest => "quest",
        EntityType::Routine => "routine",
        EntityType::FocusSession => "focus_session",
        EntityType::CheckIn => "check_in",
        EntityType::Note => "note",
        EntityType::File => "file",
        EntityType::Item => "item",
        EntityType::Location => "location",
        EntityType::ShoppingList => "shopping_list",
        EntityType::Category => "category",
    }
}

/// Edit an entity, or recreate it where the entity was erased.
pub async fn edit(
    ctx: &mut Ctx<'_>,
    id: EntityId,
    base_counter: i64,
    parts: Vec<PartWrite>,
    stated_type: Option<EntityType>,
) -> Applied<Outcome> {
    let row = available_for_write(ctx.tx, ctx.member, id, WriteScope::AnyIncludingErased)
        .await?
        .ok_or(Refusal::NotAvailable)?;

    // Type is fixed at creation. The store cannot see that it is fixed, so this
    // is the check that makes it true.
    if let Some(stated) = stated_type
        && stated != row.entity_type
    {
        return Err(Refusal::Malformed("an edit may not change an entity's type".into()).into());
    }

    if row.lifecycle == LifecycleState::Erased {
        return recreate(ctx, &row, parts).await;
    }

    let mut touching = Vec::new();
    let mut written = Vec::new();
    let counter = row.counter + 1;

    for part in &parts {
        let placement = validate_and_place(ctx, row.category_id, part).await?;
        let stored = current(ctx, &row, part).await?;
        touching.push((part.part(), part.text(), stored.text()));
        if let Some(placement) = &placement {
            let stored = current(ctx, &row, placement).await?;
            touching.push((placement.part(), placement.text(), stored.text()));
        }
        apply_part(ctx, id, part, placement.as_ref()).await?;
        written.push(part.part());
        if let Some(placement) = &placement {
            written.push(placement.part());
        }
    }

    // A stale base is never a rejection. What it can be is a conflict, and only
    // over the parts that actually overlap what moved since.
    let mut conflict = ConflictParts::default();
    if base_counter < row.counter {
        let moved = provenance::moved_since(ctx.tx, id, base_counter).await?;
        let retained = provenance::detect(&touching, &moved, ctx.member);
        for r in &retained {
            conflict.push(&r.part);
        }
        provenance::retain(ctx.tx, id, &retained, ctx.at).await?;
    }

    provenance::record(ctx.tx, id, &written, counter, ctx.member).await?;
    sqlx::query("UPDATE entity SET counter = $2, updated_at = $3 WHERE id = $1")
        .bind(id.as_uuid())
        .bind(counter)
        .bind(ctx.at)
        .execute(ctx.tx.conn())
        .await?;

    let after = read_audience(ctx, id).await?;
    changes::entity_written(
        ctx.tx,
        ctx.at,
        &EntityChange {
            entity: id,
            audience_before: Some(row.audience.clone()),
            audience_after: Some(after),
            author_before: row.author,
            author_after: row.author,
            changed_blocks: Vec::new(),
        },
    )
    .await?;

    Ok(Outcome::Applied {
        entities: vec![id],
        relations: Vec::new(),
        counter: Some(counter),
        conflict: (!conflict.is_empty()).then_some(conflict),
    })
}

/// An edit arrived for an entity that was erased.
///
/// **The erasure stands and so does the person's work.** The identity is new
/// and is stated rather than inferred, because anything that pointed at the
/// erased entity pointed at something that no longer exists.
///
/// **Audience is private to the author, whatever it was before.** A deliberate
/// departure from recreating faithfully: erasure is the affordance for
/// something that should not have been there, and a device that was offline
/// recreating it with a household audience would re-expose the thing erasure
/// exists to remove. Broadening is trivial and narrowing is unreliable, so the
/// closed default is the safe one.
///
/// **A file cannot be recreated yet.** An edit never reads `content.specific`
/// (see [`super::content::parts_written`]), so there is no `body_id` to carry
/// into the recreated entity, and [`create_file_row`] refuses for the same
/// reason a bare file create with no body does. Recreating an erased file is
/// therefore refused until an edit can name one, which is 2.2's territory —
/// not a regression, since every file create refused unconditionally before
/// this wave.
async fn recreate(
    ctx: &mut Ctx<'_>,
    tombstone: &EntityRow,
    parts: Vec<PartWrite>,
) -> Applied<Outcome> {
    let new = EntityId::from_uuid(Uuid::new_v4());
    let parts: Vec<PartWrite> = parts
        .into_iter()
        .filter(|p| !matches!(p, PartWrite::Audience(_)))
        .collect();

    let outcome = create(
        ctx,
        new,
        Some(ctx.at),
        &tombstone.capture_method,
        parts,
        tombstone.entity_type,
        None,
    )
    .await?;

    let counter = match outcome {
        Outcome::Applied { counter, .. } => counter.unwrap_or(1),
        _ => 1,
    };
    Ok(Outcome::Recreated {
        original: tombstone.id,
        new,
        counter,
    })
}

/// Put entities into the holding state, as part of one act.
///
/// **A removal that cannot land has already happened.** An identifier this
/// member cannot see, one that does not exist, and one that is already removed
/// all converge on the end state the person asked for, so none of them is a
/// refusal. Reporting one would tell them only that the thing they wanted is
/// the thing that occurred — and, in the first case, would answer a question
/// about whether something exists.
pub async fn remove(ctx: &mut Ctx<'_>, ids: &[EntityId], group: Uuid) -> Applied<Vec<EntityId>> {
    let mut removed = Vec::new();
    for id in ids {
        let Some(row) = available_for_write(ctx.tx, ctx.member, *id, WriteScope::Active).await?
        else {
            continue;
        };
        sqlx::query(
            "UPDATE entity SET lifecycle = 'deleted', deleted_at = $2, \
             deletion_group_id = $3, counter = counter + 1, updated_at = $2 WHERE id = $1",
        )
        .bind(id.as_uuid())
        .bind(ctx.at)
        .bind(group)
        .execute(ctx.tx.conn())
        .await?;

        changes::entity_written(
            ctx.tx,
            ctx.at,
            &EntityChange {
                entity: *id,
                audience_before: Some(row.audience.clone()),
                audience_after: Some(row.audience),
                author_before: row.author,
                author_after: row.author,
                changed_blocks: Vec::new(),
            },
        )
        .await?;
        removed.push(*id);
    }
    Ok(removed)
}

/// Bring an entity back, and optionally the rest of the act it was removed in.
///
/// **Restoring is never blocked by something else still being removed.** A
/// quest whose campaign is gone comes back as a quest with no campaign, which
/// is a valid thing to be.
pub async fn restore(
    ctx: &mut Ctx<'_>,
    id: EntityId,
    include_group: bool,
) -> Applied<(Vec<EntityId>, Option<Uuid>)> {
    let row = available_for_write(ctx.tx, ctx.member, id, WriteScope::Extant)
        .await?
        .ok_or(Refusal::NotAvailable)?;

    // Already active: the end state the person asked for is already true.
    if row.lifecycle == LifecycleState::Active {
        return Ok((vec![id], None));
    }

    let group = row.deletion_group_id;
    let mut rows = vec![row];
    if include_group && let Some(group) = group {
        for other in crate::store::entity::holding_group(ctx.tx, group, ctx.member).await? {
            if other.id != id {
                rows.push(other);
            }
        }
    }

    let mut restored = Vec::new();
    for row in &rows {
        sqlx::query(
            "UPDATE entity SET lifecycle = 'active', deleted_at = NULL, \
             deletion_group_id = NULL, counter = counter + 1, updated_at = $2 WHERE id = $1",
        )
        .bind(row.id.as_uuid())
        .bind(ctx.at)
        .execute(ctx.tx.conn())
        .await?;

        // Entering a container places the entity at the end, and that holds for
        // a restoration exactly as it does for a create or a move. The position
        // it had before it left is not carried back.
        if let Some(category) = row.category_id {
            let next = crate::store::entity::next_category_position(ctx.tx, category).await?;
            let part = PartWrite::CategoryPosition(Some(next));
            apply_part(ctx, row.id, &part, None).await?;
            provenance::record(ctx.tx, row.id, &[part.part()], row.counter + 1, ctx.member).await?;
        }

        changes::entity_written(
            ctx.tx,
            ctx.at,
            &EntityChange {
                entity: row.id,
                audience_before: Some(row.audience.clone()),
                audience_after: Some(row.audience.clone()),
                author_before: row.author,
                author_after: row.author,
                changed_blocks: Vec::new(),
            },
        )
        .await?;
        restored.push(row.id);
    }

    Ok((restored, include_group.then_some(group).flatten()))
}

/// Every table an entity's content lives in, other than the entity row.
///
/// **The schema's cascading deletes never fire under erasure**, and this list
/// is the consequence. Every one of them hangs off a delete of the `entity`
/// row, and erasure strips content and leaves a tombstone rather than deleting
/// it — so a cascade that reads as though it were doing the work is doing
/// nothing at all. The comment above `event_record` in migration one describing
/// its cascade as erasure describes something that never happens.
///
/// Order matters in one place: relations are removed before blocks, because a
/// relation may anchor into one.
const DEPENDENT_TABLES: &[&str] = &[
    "conflict",
    "embedding",
    "derived_text",
    "entity_version",
    "derivation_queue",
    "block",
    "entity_date",
    "entity_assignment",
    "entity_property_value",
    "event_record",
];

/// Strip an entity to a tombstone.
///
/// **The record goes before the bytes.** Bytes are the object store's and
/// arrive with Wave 2.3; the ordering is stated here because this is where it
/// will be added, and inverting it is the failure the creation rule exists to
/// prevent, lasting indefinitely.
///
/// Converges like a removal: an identifier that is not available is already in
/// the end state the person asked for.
pub async fn erase(ctx: &mut Ctx<'_>, ids: &[EntityId]) -> Applied<(Vec<EntityId>, Vec<Uuid>)> {
    let mut erased = Vec::new();
    let mut relations_gone = Vec::new();

    for id in ids {
        let Some(row) = available_for_write(ctx.tx, ctx.member, *id, WriteScope::Extant).await?
        else {
            continue;
        };

        // Erasing either endpoint removes the relation outright, which is what
        // erasure means everywhere. This is the entry easiest to miss, because
        // a relation's two endpoints are declared as cascading and the
        // declaration reads as though it were doing the work.
        let rows = sqlx::query(
            "DELETE FROM relation WHERE from_entity_id = $1 OR to_entity_id = $1 RETURNING id",
        )
        .bind(id.as_uuid())
        .fetch_all(ctx.tx.conn())
        .await?;
        for r in &rows {
            let relation: Uuid = r.try_get("id")?;
            changes::relation_gone(ctx.tx, ctx.at, relation).await?;
            relations_gone.push(relation);
        }

        for table in DEPENDENT_TABLES {
            let column = if *table == "event_record" {
                "item_id"
            } else {
                "entity_id"
            };
            let sql = format!("DELETE FROM {table} WHERE {column} = $1");
            sqlx::query(sqlx::AssertSqlSafe(sql))
                .bind(id.as_uuid())
                .execute(ctx.tx.conn())
                .await?;
        }
        provenance::erase(ctx.tx, *id).await?;

        // The side table, which holds what the type carries beyond the shared
        // model. Named by the row's own type rather than tried against all of
        // them, so a type gaining a table is one line here.
        if let Some(table) = side_table(row.entity_type) {
            let sql = format!("DELETE FROM {table} WHERE entity_id = $1");
            sqlx::query(sqlx::AssertSqlSafe(sql))
                .bind(id.as_uuid())
                .execute(ctx.tx.conn())
                .await?;
        }

        // A container that is gone holds nothing. The substrate's rule for a
        // deleted category — an entity whose category is deleted becomes
        // uncategorised, which is a valid state requiring no repair — applies a
        // fortiori to an erased one, and leaving the reference would point
        // every member of the category at a tombstone.
        //
        // The type-specific containers, the ladder and nested locations, are
        // Wave 2.2's for the same reason their content is.
        if row.entity_type == EntityType::Category {
            for orphan in crate::store::entity::uncategorise_all(ctx.tx, *id, ctx.at).await? {
                changes::entity_written(
                    ctx.tx,
                    ctx.at,
                    &EntityChange {
                        entity: orphan.id,
                        audience_before: Some(orphan.audience.clone()),
                        audience_after: Some(orphan.audience),
                        author_before: orphan.author,
                        author_after: orphan.author,
                        changed_blocks: Vec::new(),
                    },
                )
                .await?;
            }
        }

        // The tombstone. Everything a person put here is gone; what remains is
        // the identity, the type, and the authorship — which never changes —
        // so that an edit arriving from a device that was away is recognisable
        // as a recreation, and so the sequence can say the entity is gone.
        let sql = format!(
            "UPDATE entity SET title = NULL, search_text = '', capture_method = '', \
             bulk = false, category_id = NULL, category_position = NULL, \
             {AUDIENCE_COLUMN} = '{{}}', lifecycle = 'erased', deleted_at = NULL, \
             deletion_group_id = NULL, counter = counter + 1, updated_at = $2 \
             WHERE id = $1"
        );
        sqlx::query(sqlx::AssertSqlSafe(sql))
            .bind(id.as_uuid())
            .bind(ctx.at)
            .execute(ctx.tx.conn())
            .await?;

        changes::entity_gone(
            ctx.tx,
            ctx.at,
            &EntityChange {
                entity: *id,
                audience_before: Some(row.audience.clone()),
                audience_after: Some(Vec::new()),
                author_before: row.author,
                author_after: row.author,
                changed_blocks: Vec::new(),
            },
        )
        .await?;
        erased.push(*id);
    }

    Ok((erased, relations_gone))
}

fn side_table(kind: EntityType) -> Option<&'static str> {
    match kind {
        EntityType::Campaign => Some("campaign"),
        EntityType::Arc => Some("arc"),
        EntityType::Quest => Some("quest"),
        EntityType::File => Some("file"),
        EntityType::Item => Some("item"),
        EntityType::Location => Some("location"),
        EntityType::Category => Some("category"),
        EntityType::Note
        | EntityType::ShoppingList
        | EntityType::Routine
        | EntityType::FocusSession
        | EntityType::CheckIn => None,
    }
}
