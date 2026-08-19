//! The five verbs, on an entity.
//!
//! Everything here runs inside one transaction that has already taken the
//! change sequence's lock; see [`super::changes`] for why that ordering
//! matters.
//!
//! # What this item writes, and where type content goes
//!
//! The shared model — title, dates, category and the position inside it,
//! assignments, audience, bulk — is written here, one arm per part. **Type
//! content is not**, and that is a boundary rather than an omission: every
//! type-specific field goes through [`super::specific`], which dispatches to
//! the module owning the type, and a body goes through [`super::body`], which
//! divides it. Nothing about a campaign, a note, or a location is decided in
//! this file, and a lane building a domain never has to open it.
//!
//! Types are refused for reasons that expire, which is the honest answer rather
//! than an omission:
//!
//! - **A routine, a focus session, or a check-in**, until each domain is
//!   designed. The schema deliberately creates no table for them, so there is
//!   nothing to put a row in.
//! - **Content the lane behind it has not filled in**, which refuses
//!   distinguishably rather than accepting silently. See
//!   [`super::specific::not_yet_built`].
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
use super::content::{Date, FileReference, Malformed, PartWrite, Written};
use super::outcome::{ConflictParts, Outcome};
use super::parts::{ContentPart, Part};
use super::specific::{self, SpecificPart};
use super::{body, provenance};

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
        PartWrite::Specific(s) => {
            PartWrite::Specific(specific::current(ctx, row.id, row.entity_type, s).await?)
        }
    })
}

/// Apply one part to the store. Assumes it has already been validated.
///
/// `placements` are the parts the instance moved as a consequence. The shared
/// model produces at most one — the position assigned alongside a category, and
/// **the two move in one statement**, because the store's own check says a
/// category and a position are either both there or both absent: writing them in
/// sequence puts the row through a state the schema refuses, whichever order the
/// sequence is in.
///
/// A type's content may produce more than one, so this takes a slice. **Each
/// arm finds what it needs by kind rather than by position**, because the order
/// a lane returns companions in is that lane's business.
async fn apply_part(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    part: &PartWrite,
    placements: &[PartWrite],
) -> Applied<()> {
    match part {
        PartWrite::Title(v) => {
            // `search_text` is not written here. It is the title plus whatever
            // the type contributes, and the type's share needs a read of a side
            // table, so it is refreshed once after every part has landed — see
            // `refresh_search_text`.
            sqlx::query("UPDATE entity SET title = $2 WHERE id = $1")
                .bind(entity.as_uuid())
                .bind(v.as_deref())
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
            let position = placements.iter().find_map(|p| match p {
                PartWrite::CategoryPosition(v) => Some(*v),
                _ => None,
            });
            let position = position.flatten();
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
        PartWrite::Specific(s) => {
            // The companion, where there is one, is the type's own — a ladder
            // position beside a ladder parent, an assertion time beside an
            // amount — and is handed through so the module can write both in
            // one statement, for the same reason a category and its position
            // move together here.
            let companions: Vec<SpecificPart> = placements
                .iter()
                .filter_map(|p| match p {
                    PartWrite::Specific(s) => Some(s.clone()),
                    _ => None,
                })
                .collect();
            specific::apply(ctx, entity, kind, s, &companions).await?;
        }
    }
    Ok(())
}

/// Rewrite `search_text` from the title and the type's share of it.
///
/// **Maintained by the write path, not by a generated column.** A type's
/// content lives in a side table and a generated column cannot reach across
/// one. Refreshed once per write rather than per part, because the two
/// contributions land in different statements and a per-part refresh would
/// write a string that is briefly missing one of them.
///
/// `concat_ws` drops a null, so a type contributing nothing leaves the title
/// standing alone — which is the whole answer for every type today.
async fn refresh_search_text(ctx: &mut Ctx<'_>, entity: EntityId, kind: EntityType) -> Applied<()> {
    let share = specific::search_text(ctx, entity, kind).await?;
    sqlx::query("UPDATE entity SET search_text = concat_ws(' ', title, $2) WHERE id = $1")
        .bind(entity.as_uuid())
        .bind(share)
        .execute(ctx.tx.conn())
        .await?;
    Ok(())
}

/// Checks a part owes before it is applied, and the placements the instance
/// makes rather than the client.
///
/// Returns every extra part the instance moved as a consequence — a position,
/// when a category changed, and whatever a type's content moved beside the part
/// the write named.
///
/// `addressed` is every type-specific part this same message writes, handed
/// down because a type sometimes cannot decide one part without seeing another.
/// See [`specific::validate_and_place`] for the two cases that need it.
async fn validate_and_place(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    entering: Option<EntityId>,
    part: &PartWrite,
    addressed: &[SpecificPart],
) -> Applied<Vec<PartWrite>> {
    match part {
        PartWrite::Specific(s) => Ok(
            specific::validate_and_place(ctx, entity, kind, s, addressed)
                .await?
                .into_iter()
                .map(PartWrite::Specific)
                .collect(),
        ),
        PartWrite::Audience(ids) | PartWrite::Assignments(ids) => {
            if !memberships_exist(ctx.tx, ids).await? {
                return Err(Refusal::NotAvailable.into());
            }
            Ok(Vec::new())
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
                return Ok(vec![PartWrite::CategoryPosition(Some(next))]);
            }
            Ok(Vec::new())
        }
        // Leaving a container forgets the position. Nothing is carried and
        // nothing needs repair.
        PartWrite::Category(None) => Ok(vec![PartWrite::CategoryPosition(None)]),
        _ => Ok(Vec::new()),
    }
}

/// Every type-specific part one message writes.
///
/// Built once per write rather than per part: it is the same set for every part
/// in the loop, and a type asking about a sibling field is asking about this.
fn addressed_specifics(written: &Written) -> Vec<SpecificPart> {
    written
        .parts
        .iter()
        .filter_map(|p| match p {
            PartWrite::Specific(s) => Some(s.clone()),
            _ => None,
        })
        .collect()
}

/// The side-table row a type carries beyond the shared model, with its
/// defaults.
///
/// Which table that is comes from [`specific::side_table`], which is also what
/// erasure removes and what the column helpers write through, so a type gaining
/// a table is one line in one place. `None` there means a note or a shopping
/// list, whose content is a body and nothing else.
///
/// **A file is the one type this does not insert a bare default row for.**
/// [`create_file_row`] inserts it instead, because the row cannot exist
/// without a `body_id` and nothing here has one; `media_type` still lands
/// through the ordinary parts loop below, exactly as every other type's
/// columns do.
async fn make_type_row(
    ctx: &mut Ctx<'_>,
    entity: EntityId,
    kind: EntityType,
    file: Option<FileReference>,
) -> Applied<()> {
    match kind {
        EntityType::File => return create_file_row(ctx, entity, file).await,
        EntityType::Routine | EntityType::FocusSession | EntityType::CheckIn => {
            return Err(Refusal::Malformed(
                "this type's content is not yet designed and the store has no table for it".into(),
            )
            .into());
        }
        _ => {}
    }
    let Some(table) = specific::side_table(kind) else {
        return Ok(());
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
///
/// **`media_type` is not written here.** It is an ordinary column of this
/// same row, served through [`super::specific::knowledge`] like any other
/// type-specific field, and the parts loop in [`create`] applies it right
/// after this row exists. Writing it twice would mean nothing wrong — both
/// writes carry the same value, read from the same message — but it would
/// mean this function reaching past what only it can decide: whether the
/// named body exists and how large it is.
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

    // A length no `bigint` column can hold is not a size any real body has —
    // clamping it would silently record a wrong number, so it is refused
    // instead of guessed at.
    let byte_size = i64::try_from(body.len).map_err(|_| {
        Refusal::Malformed("a file names a body too large for this store to record".into())
    })?;

    sqlx::query("INSERT INTO file (entity_id, body_id, byte_size) VALUES ($1, $2, $3)")
        .bind(entity.as_uuid())
        .bind(file.body_id)
        .bind(byte_size)
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
    written: Written,
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
    let stated_audience = written
        .parts
        .iter()
        .any(|p| matches!(p, PartWrite::Audience(_)));
    let category = written.parts.iter().find_map(|p| match p {
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

    let addressed = addressed_specifics(&written);
    let mut moved = Vec::new();
    for part in &written.parts {
        let placements = validate_and_place(ctx, id, kind, None, part, &addressed).await?;
        apply_part(ctx, id, kind, part, &placements).await?;
        moved.push(part.part());
        for placement in &placements {
            moved.push(placement.part());
        }
    }

    // A body is many parts and they are not known until it is divided against
    // what is stored, which on a create is nothing.
    //
    // `Creating` is what withholds bulk graduation: a body arriving with a
    // creation was captured rather than authored, and nothing readable from
    // inside `body::apply` distinguishes the two. See `body::Occasion`, which
    // records why each of the obvious in-file signals is silently wrong.
    let mut changed_blocks = Vec::new();
    if let Some(text) = &written.body {
        let touch = body::apply(ctx, id, kind, text, body::Occasion::Creating).await?;
        moved.extend(touch.written);
        changed_blocks.extend(touch.changed);
    }

    if !stated_audience && let Some(category) = category {
        let default = creation_default_audience(ctx, category).await?;
        if !default.is_empty() {
            if !memberships_exist(ctx.tx, &default).await? {
                return Err(Refusal::NotAvailable.into());
            }
            let part = PartWrite::Audience(default);
            apply_part(ctx, id, kind, &part, &[]).await?;
            moved.push(part.part());
        }
    }

    refresh_search_text(ctx, id, kind).await?;
    provenance::record(ctx.tx, id, &moved, 1, ctx.member).await?;

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
            changed_blocks,
        },
    )
    .await?;

    Ok(Outcome::applied_entity(id, 1))
}

async fn read_audience(ctx: &mut Ctx<'_>, id: EntityId) -> Applied<Vec<MemberRef>> {
    let row = available_for_write(ctx.tx, ctx.member, id, WriteScope::AnyIncludingErased).await?;
    Ok(row.map(|r| r.audience).unwrap_or_default())
}

/// The store's spelling of a type, which is also the one a refusal says out
/// loud to a person reading a log.
#[must_use]
pub fn type_name(kind: EntityType) -> &'static str {
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
    written: Written,
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
        return recreate(ctx, &row, written).await;
    }

    let kind = row.entity_type;
    let mut touching: Vec<(Part, Option<String>, Option<String>)> = Vec::new();
    let mut moved = Vec::new();
    let counter = row.counter + 1;

    let addressed = addressed_specifics(&written);
    for part in &written.parts {
        let placements =
            validate_and_place(ctx, id, kind, row.category_id, part, &addressed).await?;
        let stored = current(ctx, &row, part).await?;
        touching.push((part.part(), part.text(), stored.text()));
        // **A write that changed nothing did not move the part.** The same
        // comparison that decides a conflict decides this, and it has to: a
        // part's provenance row carries *who* last moved it, so recording a
        // no-op would hand ownership to a member who wrote nothing and erase the
        // one who did. A later conflict would then name the wrong person, and
        // `conflict.theirs_member_id` is stored and crosses the wire — the
        // substrate requires that whose edit each value was is known and may be
        // shown, and after a same-value write it would be known and wrong.
        //
        // The entity's own counter still advances below, which is what tells a
        // client there was a write at all. Only the per-part record is withheld,
        // and a part a write does not address already goes unrecorded — so this
        // is the existing shape rather than a new rule.
        //
        // `specific::guidance` already does this in the upward climb, reading the
        // current state and returning before recording when nothing moves. The
        // general edit path now agrees with it.
        let mut record = |part: &PartWrite, stored: &PartWrite| {
            if part.text() != stored.text() {
                moved.push(part.part());
            }
        };
        record(part, &stored);
        // Each companion is compared like any other part, and against what the
        // store holds *now* — which is why this runs before the write below.
        for placement in &placements {
            let stored = current(ctx, &row, placement).await?;
            touching.push((placement.part(), placement.text(), stored.text()));
            record(placement, &stored);
        }
        apply_part(ctx, id, kind, part, &placements).await?;
    }

    // A body reads what is stored and writes what changed in one step, because
    // which blocks a body write touches is decided by the match between the two
    // and cannot be asked before the write is prepared.
    //
    // A write that states `bulk` withholds graduation, because the person said
    // what they wanted and a derived value does not outrank an authored one.
    // The parts above have already been applied, so without this the statement
    // would be overwritten a few lines later by a rule meant to guess at it.
    let states_bulk = written
        .parts
        .iter()
        .any(|p| matches!(p, PartWrite::Bulk(_)));
    let mut changed_blocks = Vec::new();
    if let Some(text) = &written.body {
        let touch =
            body::apply(ctx, id, kind, text, body::Occasion::Editing { states_bulk }).await?;
        touching.extend(touch.touching);
        moved.extend(touch.written);
        changed_blocks.extend(touch.changed);
    }

    refresh_search_text(ctx, id, kind).await?;

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

    provenance::record(ctx.tx, id, &moved, counter, ctx.member).await?;
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
            changed_blocks,
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
/// **A file cannot be recreated yet.** `body_id` is the one field
/// `content::parts_written` never reaches, on either arm — see its doc — so an
/// edit's `written` never carries one, however the message named. There is
/// therefore no `body_id` to carry into the recreated entity, and
/// [`create_file_row`] refuses for the same reason a bare file create with no
/// body does. Not a regression: every file create refused unconditionally
/// before this wave.
async fn recreate(
    ctx: &mut Ctx<'_>,
    tombstone: &EntityRow,
    mut written: Written,
) -> Applied<Outcome> {
    let new = EntityId::from_uuid(Uuid::new_v4());
    written
        .parts
        .retain(|p| !matches!(p, PartWrite::Audience(_)));

    let outcome = create(
        ctx,
        new,
        Some(ctx.at),
        &tombstone.capture_method,
        written,
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
            apply_part(ctx, row.id, row.entity_type, &part, &[]).await?;
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
        if let Some(table) = specific::side_table(row.entity_type) {
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
        // **That is category *membership*, and it is only half of what a
        // category holds.** The other half is category *nesting* —
        // `category.parent_category_id` — and `uncategorise_all` does not touch
        // it. An earlier version of this comment named "the ladder and nested
        // locations" and omitted the third case, which is where the blind spot
        // started; it is spelled out here so the next reader does not inherit
        // it.
        //
        // The type-specific containers — the ladder, nested locations, and
        // nested categories — release through the type-content seam, because
        // the column that points at the container is a column of the type's own
        // side table. The seam hands back identities and which part of each
        // moved; the counter and the audience a change entry needs come from the
        // store layer, which is the only place entitled to read that column. See
        // `specific::detach_contained`.
        //
        // **A type that has not built its release must not look like an empty
        // one.** `Detachment::NotBuilt` is a distinct answer for exactly that
        // reason, and it is stepped over here rather than treated as nothing to
        // do. Two types still answer that way and both are visible rather than
        // silent: Guidance's arcs and quests, and a nested category.
        let contained = match specific::detach_contained(ctx, *id, row.entity_type).await? {
            specific::Detachment::Released(released) => released,
            specific::Detachment::NoContainer | specific::Detachment::NotBuilt => Vec::new(),
        };
        let ids: Vec<EntityId> = contained.iter().map(|r| r.entity).collect();
        for orphan in crate::store::entity::detached_from_container(ctx.tx, &ids, ctx.at).await? {
            // **A release is a write, so it owes provenance.** The counter
            // advanced, and a part that moves a counter without recording which
            // part moved is invisible to conflict detection: a later stale edit
            // to the same field would merge silently, losing the fact that the
            // container went away underneath it. `restore` already records the
            // position it moves as a consequence of something else; this is the
            // same shape and now gets the same treatment.
            //
            // Attributed to the member who erased the container, because that is
            // who caused it.
            for r in contained.iter().filter(|r| r.entity == orphan.id) {
                provenance::record(
                    ctx.tx,
                    orphan.id,
                    std::slice::from_ref(&r.part),
                    orphan.counter,
                    ctx.member,
                )
                .await?;
            }
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

        if row.entity_type == EntityType::Category {
            for orphan in crate::store::entity::uncategorise_all(ctx.tx, *id, ctx.at).await? {
                // The same reasoning, for the membership half. Two parts move —
                // the category and the position within it — and both are
                // recorded, because the counter advanced and something has to be
                // able to say what changed.
                //
                // **The alternative reading was considered and rejected**: that
                // uncategorising is a consequence rather than an authored
                // placement and so should not conflict. It cannot be had for
                // free — the counter already advances here, and a write that
                // moves a counter and records nothing is the silent case. Either
                // it is a write to this entity or it is not; it is, so it pays.
                provenance::record(
                    ctx.tx,
                    orphan.id,
                    &[
                        Part::Content(ContentPart::Category),
                        Part::Content(ContentPart::CategoryPosition),
                    ],
                    orphan.counter,
                    ctx.member,
                )
                .await?;
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
