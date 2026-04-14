use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::auth::models::AuthUser;
use crate::contracts::EntityType;
use crate::error::AppError;
use crate::sync::models::{MutationEnvelope, MutationResult, MutationStatus, Operation};

// ---------------------------------------------------------------------------
// S007: ConflictOutcome
// ---------------------------------------------------------------------------

/// Outcome of a conflict check against the current database state.
pub enum ConflictOutcome {
    /// No conflict: row does not exist (create path) or base_version matches.
    Clean,
    /// Conflict detected; LWW chose incoming (occurred_at >= current updated_at).
    /// Conflict should still be recorded.
    Accepted { current_payload: serde_json::Value },
    /// Conflict detected; LWW chose existing (occurred_at < current updated_at).
    Rejected { current_payload: serde_json::Value },
}

// ---------------------------------------------------------------------------
// S005: entity_type_to_table
// ---------------------------------------------------------------------------

/// Map an EntityType to its Postgres table name.
/// Exhaustive over all 18 variants.
pub fn entity_type_to_table(entity_type: &EntityType) -> &'static str {
    match entity_type {
        EntityType::User => "users",
        EntityType::Household => "households",
        EntityType::Initiative => "initiatives",
        EntityType::Tag => "tags",
        EntityType::Attachment => "attachments",
        EntityType::GuidanceEpic => "guidance_epics",
        EntityType::GuidanceQuest => "guidance_quests",
        EntityType::GuidanceRoutine => "guidance_routines",
        EntityType::GuidanceFocusSession => "guidance_focus_sessions",
        EntityType::GuidanceDailyCheckin => "guidance_daily_checkins",
        EntityType::KnowledgeNote => "knowledge_notes",
        EntityType::KnowledgeNoteSnapshot => "knowledge_note_snapshots",
        EntityType::TrackingLocation => "tracking_locations",
        EntityType::TrackingCategory => "tracking_categories",
        EntityType::TrackingItem => "tracking_items",
        EntityType::TrackingItemEvent => "tracking_item_events",
        EntityType::TrackingShoppingList => "tracking_shopping_lists",
        EntityType::TrackingShoppingListItem => "tracking_shopping_list_items",
    }
}

// ---------------------------------------------------------------------------
// S005: check_ownership
// ---------------------------------------------------------------------------

/// Returns Ok(()) if the authenticated user owns (or is a member of the household that owns)
/// the specified entity. Returns AppError::Forbidden if the check fails or the row is not found.
///
/// Ownership rules differ by table schema:
/// - User-scoped tables (user_id column): SELECT user_id FROM <table> WHERE id = $1
/// - Household-scoped tables (household_id column): verify via household_memberships
/// - Special cases: users (id IS the user), households (owner_id), indirectly-owned tables
pub async fn check_ownership(
    db: &PgPool,
    entity_type: &EntityType,
    entity_id: Uuid,
    auth_user: &AuthUser,
) -> Result<(), AppError> {
    match entity_type {
        // ---- Self-referential: the entity IS the user ----
        EntityType::User => {
            if entity_id != auth_user.user_id {
                return Err(AppError::Forbidden);
            }
            Ok(())
        }

        // ---- Household: check owner_id ----
        EntityType::Household => {
            let row: Option<(Uuid,)> =
                sqlx::query_as("SELECT owner_id FROM households WHERE id = $1")
                    .bind(entity_id)
                    .fetch_optional(db)
                    .await
                    .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

            match row {
                Some((owner_id,)) if owner_id == auth_user.user_id => Ok(()),
                _ => Err(AppError::Forbidden),
            }
        }

        // ---- User-scoped tables with direct user_id column ----
        EntityType::Initiative
        | EntityType::Tag
        | EntityType::Attachment
        | EntityType::GuidanceEpic
        | EntityType::GuidanceQuest
        | EntityType::GuidanceRoutine
        | EntityType::GuidanceFocusSession
        | EntityType::GuidanceDailyCheckin
        | EntityType::KnowledgeNote => {
            let table = entity_type_to_table(entity_type);
            let sql = format!("SELECT user_id FROM {table} WHERE id = $1");
            let row: Option<(Uuid,)> = sqlx::query_as(&sql)
                .bind(entity_id)
                .fetch_optional(db)
                .await
                .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

            match row {
                Some((user_id,)) if user_id == auth_user.user_id => Ok(()),
                _ => Err(AppError::Forbidden),
            }
        }

        // ---- KnowledgeNoteSnapshot: immutable, linked via note_id → knowledge_notes ----
        EntityType::KnowledgeNoteSnapshot => {
            let row: Option<(Uuid,)> = sqlx::query_as(
                "SELECT kn.user_id \
                 FROM knowledge_note_snapshots kns \
                 JOIN knowledge_notes kn ON kn.id = kns.note_id \
                 WHERE kns.id = $1",
            )
            .bind(entity_id)
            .fetch_optional(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

            match row {
                Some((user_id,)) if user_id == auth_user.user_id => Ok(()),
                _ => Err(AppError::Forbidden),
            }
        }

        // ---- Household-scoped tables: verify via household_memberships ----
        EntityType::TrackingLocation
        | EntityType::TrackingCategory
        | EntityType::TrackingShoppingList => {
            let table = entity_type_to_table(entity_type);
            let sql = format!(
                "SELECT 1 \
                 FROM {table} t \
                 JOIN household_memberships hm ON hm.household_id = t.household_id \
                 WHERE t.id = $1 AND hm.user_id = $2 AND hm.deleted_at IS NULL"
            );
            let row: Option<(i32,)> = sqlx::query_as(&sql)
                .bind(entity_id)
                .bind(auth_user.user_id)
                .fetch_optional(db)
                .await
                .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

            if row.is_some() {
                Ok(())
            } else {
                Err(AppError::Forbidden)
            }
        }

        // ---- TrackingItem: has both user_id and household_id; accept either ----
        EntityType::TrackingItem => {
            let row: Option<(Option<Uuid>, Option<Uuid>)> =
                sqlx::query_as("SELECT user_id, household_id FROM tracking_items WHERE id = $1")
                    .bind(entity_id)
                    .fetch_optional(db)
                    .await
                    .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

            match row {
                None => Err(AppError::Forbidden),
                Some((user_id, household_id)) => {
                    // Accept if directly owned
                    if user_id == Some(auth_user.user_id) {
                        return Ok(());
                    }
                    // Accept if household member
                    if let Some(hid) = household_id {
                        let member: Option<(i32,)> = sqlx::query_as(
                            "SELECT 1 FROM household_memberships \
                             WHERE household_id = $1 AND user_id = $2 AND deleted_at IS NULL",
                        )
                        .bind(hid)
                        .bind(auth_user.user_id)
                        .fetch_optional(db)
                        .await
                        .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;
                        if member.is_some() {
                            return Ok(());
                        }
                    }
                    Err(AppError::Forbidden)
                }
            }
        }

        // ---- TrackingItemEvent: linked via item_id → tracking_items ----
        EntityType::TrackingItemEvent => {
            let row: Option<(Option<Uuid>, Option<Uuid>)> = sqlx::query_as(
                "SELECT ti.user_id, ti.household_id \
                 FROM tracking_item_events te \
                 JOIN tracking_items ti ON ti.id = te.item_id \
                 WHERE te.id = $1",
            )
            .bind(entity_id)
            .fetch_optional(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

            match row {
                None => Err(AppError::Forbidden),
                Some((user_id, household_id)) => {
                    if user_id == Some(auth_user.user_id) {
                        return Ok(());
                    }
                    if let Some(hid) = household_id {
                        let member: Option<(i32,)> = sqlx::query_as(
                            "SELECT 1 FROM household_memberships \
                             WHERE household_id = $1 AND user_id = $2 AND deleted_at IS NULL",
                        )
                        .bind(hid)
                        .bind(auth_user.user_id)
                        .fetch_optional(db)
                        .await
                        .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;
                        if member.is_some() {
                            return Ok(());
                        }
                    }
                    Err(AppError::Forbidden)
                }
            }
        }

        // ---- TrackingShoppingListItem: linked via shopping_list_id → tracking_shopping_lists ----
        EntityType::TrackingShoppingListItem => {
            let row: Option<(i32,)> = sqlx::query_as(
                "SELECT 1 \
                 FROM tracking_shopping_list_items sli \
                 JOIN tracking_shopping_lists sl ON sl.id = sli.shopping_list_id \
                 JOIN household_memberships hm ON hm.household_id = sl.household_id \
                 WHERE sli.id = $1 AND hm.user_id = $2 AND hm.deleted_at IS NULL",
            )
            .bind(entity_id)
            .bind(auth_user.user_id)
            .fetch_optional(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

            if row.is_some() {
                Ok(())
            } else {
                Err(AppError::Forbidden)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// S007: detect_conflict
// ---------------------------------------------------------------------------

/// Determines whether a conflict exists for the given entity, and if so,
/// whether LWW rules would accept or reject the incoming mutation.
///
/// Must be called within an open transaction so the `FOR UPDATE` lock is held.
///
/// # Conflict logic
/// - Row not found → `Clean` (create path; no conflict possible)
/// - `base_version` is `None` and operation is not Create → treat as LWW with `occurred_at`
/// - `base_version` is `Some` and `current.updated_at <= base_version` → `Clean` (no concurrent edit)
/// - `base_version` is `Some` and `current.updated_at > base_version` → conflict:
///   - `occurred_at >= current.updated_at` → `Accepted` (LWW: incoming wins)
///   - `occurred_at < current.updated_at`  → `Rejected` (LWW: existing wins)
///
/// Note: `tracking_item_events` and `knowledge_note_snapshots` have no `updated_at` column;
/// callers must bypass this function for those entity types.
pub async fn detect_conflict(
    tx: &mut Transaction<'_, Postgres>,
    entity_type: &EntityType,
    entity_id: Uuid,
    base_version: Option<DateTime<Utc>>,
    occurred_at: DateTime<Utc>,
) -> Result<ConflictOutcome, AppError> {
    let table = entity_type_to_table(entity_type);
    let sql = format!(
        "SELECT updated_at, row_to_json({table}.*) AS payload \
         FROM {table} WHERE id = $1 FOR UPDATE"
    );

    let row: Option<(DateTime<Utc>, serde_json::Value)> = sqlx::query_as(&sql)
        .bind(entity_id)
        .fetch_optional(tx.as_mut())
        .await
        .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

    let (current_updated_at, current_payload) = match row {
        None => return Ok(ConflictOutcome::Clean),
        Some(r) => r,
    };

    // Resolve effective base_version; None on non-create is treated as LWW with occurred_at.
    let effective_base = match base_version {
        Some(bv) => bv,
        None => {
            // No version info: apply LWW — occurred_at vs current updated_at.
            if occurred_at >= current_updated_at {
                return Ok(ConflictOutcome::Accepted { current_payload });
            } else {
                return Ok(ConflictOutcome::Rejected { current_payload });
            }
        }
    };

    // No conflict: client's base is current.
    if current_updated_at <= effective_base {
        return Ok(ConflictOutcome::Clean);
    }

    // Conflict detected.
    if occurred_at >= current_updated_at {
        Ok(ConflictOutcome::Accepted { current_payload })
    } else {
        Ok(ConflictOutcome::Rejected { current_payload })
    }
}

// ---------------------------------------------------------------------------
// S009: check_quantity_conflict
// ---------------------------------------------------------------------------

/// Returns true if the payload contains a "quantity" or "consumed_quantity" key.
/// Used to gate the hard-reject path for TrackingItemEvent quantity conflicts.
pub fn check_quantity_conflict(payload: &serde_json::Value) -> bool {
    if let Some(obj) = payload.as_object() {
        obj.contains_key("quantity") || obj.contains_key("consumed_quantity")
    } else {
        false
    }
}

// ---------------------------------------------------------------------------
// S006: record_sync_mutation
// ---------------------------------------------------------------------------

/// INSERT a row into `sync_mutations` as a dedup record for this mutation.
/// Called inside the per-mutation transaction, after the mutation is applied.
async fn record_sync_mutation(
    tx: &mut Transaction<'_, Postgres>,
    envelope: &MutationEnvelope,
    user_id: Uuid,
) -> Result<(), AppError> {
    // Serialize entity_type and operation to their canonical string forms.
    let entity_type_str = serde_json::to_value(&envelope.entity_type)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_owned()))
        .unwrap_or_default();
    let operation_str = serde_json::to_value(&envelope.operation)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_owned()))
        .unwrap_or_default();

    sqlx::query(
        "INSERT INTO sync_mutations \
         (mutation_id, device_id, user_id, entity_type, entity_id, operation) \
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(envelope.mutation_id)
    .bind(envelope.device_id)
    .bind(user_id)
    .bind(&entity_type_str)
    .bind(envelope.entity_id)
    .bind(&operation_str)
    .execute(tx.as_mut())
    .await
    .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

    Ok(())
}

// ---------------------------------------------------------------------------
// S008: create_conflict_copy
// ---------------------------------------------------------------------------

/// Creates a conflict copy note when a KnowledgeNote update conflicts.
///
/// Steps:
/// 1. INSERT a new knowledge_notes row (new id, payload fields, user_id, now() timestamps)
/// 2. INSERT into entity_relations linking new note → original (relation_type = "duplicates")
/// 3. INSERT into sync_conflicts (resolution = "conflict_copy")
/// 4. Return the new note Uuid
pub async fn create_conflict_copy(
    tx: &mut Transaction<'_, Postgres>,
    original_entity_id: Uuid,
    payload: &serde_json::Value,
    user_id: Uuid,
    mutation_id: Uuid,
    base_version: Option<DateTime<Utc>>,
    current_version: Option<DateTime<Utc>>,
) -> Result<Uuid, AppError> {
    let new_id = Uuid::new_v4();

    // Extract fields from payload with fallbacks.
    let title = payload
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("Conflict copy");
    let content = payload.get("content").and_then(|v| v.as_str());
    let initiative_id: Option<Uuid> = payload
        .get("initiative_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok());

    // 1. Insert new knowledge_notes row.
    sqlx::query(
        "INSERT INTO knowledge_notes \
         (id, title, content, user_id, initiative_id, created_at, updated_at) \
         VALUES ($1, $2, $3, $4, $5, now(), now())",
    )
    .bind(new_id)
    .bind(title)
    .bind(content)
    .bind(user_id)
    .bind(initiative_id)
    .execute(tx.as_mut())
    .await
    .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

    // 2. Link new note → original via entity_relations (relation_type = "duplicates").
    sqlx::query(
        "INSERT INTO entity_relations \
         (from_entity_type, from_entity_id, to_entity_type, to_entity_id, \
          relation_type, source_type, status, user_id) \
         VALUES ('knowledge_note', $1, 'knowledge_note', $2, 'duplicates', 'sync', 'accepted', $3)",
    )
    .bind(new_id)
    .bind(original_entity_id)
    .bind(user_id)
    .execute(tx.as_mut())
    .await
    .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

    // 3. Record sync_conflicts row with resolution = "conflict_copy".
    let current_payload: Option<serde_json::Value> = None; // caller can pass if needed
    sqlx::query(
        "INSERT INTO sync_conflicts \
         (mutation_id, entity_type, entity_id, base_version, current_version, \
          incoming_payload, current_payload, resolution, user_id) \
         VALUES ($1, 'knowledge_note', $2, $3, $4, $5, $6, 'conflict_copy', $7)",
    )
    .bind(mutation_id)
    .bind(original_entity_id)
    .bind(base_version)
    .bind(current_version)
    .bind(payload)
    .bind(current_payload)
    .bind(user_id)
    .execute(tx.as_mut())
    .await
    .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

    Ok(new_id)
}

// ---------------------------------------------------------------------------
// S006: apply_mutations
// ---------------------------------------------------------------------------

/// Process a batch of client mutations serially.
///
/// Each mutation is processed independently within its own transaction.
/// Returns a per-mutation result. Returns Err on unrecoverable errors
/// (e.g., Forbidden ownership violation or quantity conflict).
pub async fn apply_mutations(
    db: &PgPool,
    mutations: Vec<MutationEnvelope>,
    auth_user: AuthUser,
) -> Result<Vec<MutationResult>, AppError> {
    let mut results = Vec::with_capacity(mutations.len());

    for envelope in mutations {
        let result = process_single_mutation(db, envelope, &auth_user).await?;
        results.push(result);
    }

    Ok(results)
}

/// Process a single MutationEnvelope. Called by apply_mutations.
///
/// Returns the MutationResult for this mutation, or Err on Forbidden/Conflict errors.
async fn process_single_mutation(
    db: &PgPool,
    envelope: MutationEnvelope,
    auth_user: &AuthUser,
) -> Result<MutationResult, AppError> {
    // Step 1: Dedup check — if mutation_id already applied, skip.
    let already_applied: Option<(Uuid,)> =
        sqlx::query_as("SELECT mutation_id FROM sync_mutations WHERE mutation_id = $1")
            .bind(envelope.mutation_id)
            .fetch_optional(db)
            .await
            .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

    if already_applied.is_some() {
        return Ok(MutationResult {
            mutation_id: envelope.mutation_id,
            status: MutationStatus::Deduplicated,
        });
    }

    // Step 2: Ownership check — skip for Create (row doesn't exist yet).
    // For Update/Delete, the row must already exist and must be owned by auth_user.
    if !matches!(envelope.operation, Operation::Create) {
        check_ownership(db, &envelope.entity_type, envelope.entity_id, auth_user).await?;
    }

    // Step 3: Begin transaction.
    let mut tx = db
        .begin()
        .await
        .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

    // Step 4+5+6: Dispatch by operation.
    let mutation_result = dispatch_mutation(&mut tx, &envelope, auth_user).await;

    match mutation_result {
        Ok(status) => {
            // Step 6: Record to sync_mutations.
            record_sync_mutation(&mut tx, &envelope, auth_user.user_id).await?;
            tx.commit()
                .await
                .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;
            Ok(MutationResult {
                mutation_id: envelope.mutation_id,
                status,
            })
        }
        Err(e) => {
            // Rollback on any error (Forbidden, Conflict, Internal).
            let _ = tx.rollback().await;
            Err(e)
        }
    }
}

/// Dispatch a mutation to the correct operation handler within an open transaction.
async fn dispatch_mutation(
    tx: &mut Transaction<'_, Postgres>,
    envelope: &MutationEnvelope,
    auth_user: &AuthUser,
) -> Result<MutationStatus, AppError> {
    match &envelope.operation {
        Operation::Create => apply_create(tx, envelope, auth_user).await,
        Operation::Update => apply_update(tx, envelope, auth_user).await,
        Operation::Delete => apply_delete(tx, envelope).await,
    }
}

/// Handle a Create mutation.
///
/// For create, we use the payload to build an INSERT. If the row already exists
/// (duplicate create), we treat it as idempotent and return Accepted.
async fn apply_create(
    tx: &mut Transaction<'_, Postgres>,
    envelope: &MutationEnvelope,
    auth_user: &AuthUser,
) -> Result<MutationStatus, AppError> {
    let table = entity_type_to_table(&envelope.entity_type);

    // Check if row already exists (idempotent create).
    let sql_check = format!("SELECT id FROM {table} WHERE id = $1");
    let existing: Option<(Uuid,)> = sqlx::query_as(&sql_check)
        .bind(envelope.entity_id)
        .fetch_optional(tx.as_mut())
        .await
        .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

    if existing.is_some() {
        // Idempotent: row already exists, return Accepted.
        return Ok(MutationStatus::Accepted);
    }

    // Insert new row: use entity_id as id, set user_id and timestamps.
    // We use a generic INSERT using the payload JSONB for content fields.
    // Per Tech.md guidance, for create we insert id, user_id, and delegate
    // content fields to a per-entity approach. For this feature, we use a
    // minimal safe insert: id + user_id + timestamps via payload columns.
    //
    // Special handling: some tables don't have a user_id column directly
    // (household-scoped). For those, we insert using the payload as-is,
    // overriding id. The caller (client connector) is responsible for
    // providing a valid payload.
    //
    // For simplicity and safety, we store the payload directly and use
    // jsonb_populate_record approach via a raw INSERT from the JSON payload.
    // We override `id` to ensure it matches entity_id.
    insert_from_payload(
        tx,
        table,
        &envelope.entity_type,
        envelope.entity_id,
        auth_user.user_id,
        envelope.payload.as_ref(),
    )
    .await?;

    Ok(MutationStatus::Accepted)
}

/// Insert a new row from a payload JSONB value.
///
/// Strategy: build an INSERT using the payload's keys as column names.
/// Always override `id` to entity_id. For user-scoped tables, override `user_id`.
/// For household-scoped tables, preserve `household_id` from payload.
///
/// This is a "best effort" generic insert: the payload must conform to the
/// table schema. Any unknown columns will cause a DB error (surfaced as Internal).
async fn insert_from_payload(
    tx: &mut Transaction<'_, Postgres>,
    table: &str,
    entity_type: &EntityType,
    entity_id: Uuid,
    user_id: Uuid,
    payload: Option<&serde_json::Value>,
) -> Result<(), AppError> {
    let payload_obj = payload
        .and_then(|p| p.as_object())
        .cloned()
        .unwrap_or_default();

    // Determine which "server-controlled" columns to inject/override.
    let is_user_scoped = matches!(
        entity_type,
        EntityType::Initiative
            | EntityType::Tag
            | EntityType::Attachment
            | EntityType::GuidanceEpic
            | EntityType::GuidanceQuest
            | EntityType::GuidanceRoutine
            | EntityType::GuidanceFocusSession
            | EntityType::GuidanceDailyCheckin
            | EntityType::KnowledgeNote
            | EntityType::KnowledgeNoteSnapshot
            | EntityType::TrackingItem
    );

    // Build column list and placeholder list from payload keys.
    // We always include `id`. Add `user_id` for user-scoped tables.
    // Exclude server-managed columns from payload to avoid conflicts.
    let excluded_keys: &[&str] = &["id", "user_id", "created_at", "updated_at", "deleted_at"];

    // The first N columns are typed binds (Uuid). Track how many precede JSON values.
    let mut typed_uuid_count: usize = 1; // always: id
    let mut columns: Vec<String> = vec!["id".to_owned()];
    let mut json_values: Vec<serde_json::Value> = vec![];

    if is_user_scoped {
        columns.push("user_id".to_owned());
        typed_uuid_count += 1;
    }

    for (key, val) in &payload_obj {
        if excluded_keys.contains(&key.as_str()) {
            continue;
        }
        columns.push(key.clone());
        json_values.push(val.clone());
    }

    if columns.is_empty() {
        return Err(AppError::BadRequest("Create payload is empty".to_string()));
    }

    let col_list = columns.join(", ");
    let placeholders: Vec<String> = (1..=columns.len()).map(|i| format!("${i}")).collect();
    let placeholder_list = placeholders.join(", ");

    let sql = format!("INSERT INTO {table} ({col_list}) VALUES ({placeholder_list})");

    // Bind typed Uuid parameters first (avoids TEXT→UUID cast failures in Postgres).
    let mut query = sqlx::query(&sql).bind(entity_id);
    if typed_uuid_count > 1 {
        // user_id is the second typed param
        query = query.bind(user_id);
    }
    // Then bind the remaining JSON-derived values.
    for val in &json_values {
        query = bind_json_value(query, val);
    }

    query
        .execute(tx.as_mut())
        .await
        .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

    Ok(())
}

/// Bind a serde_json::Value to a sqlx query as a typed parameter.
fn bind_json_value<'q>(
    query: sqlx::query::Query<'q, Postgres, sqlx::postgres::PgArguments>,
    val: &'q serde_json::Value,
) -> sqlx::query::Query<'q, Postgres, sqlx::postgres::PgArguments> {
    match val {
        serde_json::Value::Null => query.bind(None::<String>),
        serde_json::Value::Bool(b) => query.bind(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                query.bind(i)
            } else if let Some(f) = n.as_f64() {
                query.bind(f)
            } else {
                query.bind(n.to_string())
            }
        }
        serde_json::Value::String(s) => query.bind(s.as_str()),
        // Objects/arrays: bind as JSONB text
        _ => query.bind(val.to_string()),
    }
}

/// Handle an Update mutation.
///
/// For the general case, performs a safe update: `SET updated_at = now()`.
/// Full payload merge is out of scope for this feature per Tech.md.
///
/// Special-cased entity types that lack `updated_at` (TrackingItemEvent,
/// KnowledgeNoteSnapshot) receive no update (these are append-only/immutable).
///
/// Conflict detection is run before applying the update.
async fn apply_update(
    tx: &mut Transaction<'_, Postgres>,
    envelope: &MutationEnvelope,
    auth_user: &AuthUser,
) -> Result<MutationStatus, AppError> {
    // TrackingItemEvent is append-only: no updated_at. Conflict detection is
    // handled via the quantity conflict path (S009). Here we simply accept.
    if matches!(envelope.entity_type, EntityType::TrackingItemEvent) {
        // S009: Check quantity conflict before accepting.
        // Conflict detection is bypassed (no updated_at), so we always hit quantity check.
        if let Some(payload) = &envelope.payload {
            if check_quantity_conflict(payload) {
                // Per S009: reject with Conflict so the client re-reads and resubmits.
                return Err(AppError::Conflict(
                    "quantity conflict — re-read and resubmit".to_string(),
                ));
            }
        }
        // No conflict check possible; apply LWW accept.
        // For append-only tables, "update" semantics are not applicable.
        // We record the mutation but don't actually modify the row.
        return Ok(MutationStatus::Accepted);
    }

    // KnowledgeNoteSnapshot is immutable: reject updates.
    if matches!(envelope.entity_type, EntityType::KnowledgeNoteSnapshot) {
        return Err(AppError::BadRequest(
            "knowledge_note_snapshot is immutable".to_string(),
        ));
    }

    // Step 4: Conflict detection (for tables with updated_at).
    let outcome = detect_conflict(
        tx,
        &envelope.entity_type,
        envelope.entity_id,
        envelope.base_version,
        envelope.occurred_at,
    )
    .await?;

    match outcome {
        ConflictOutcome::Clean => {
            // No conflict; apply the update.
            apply_update_to_table(tx, &envelope.entity_type, envelope.entity_id).await?;
            Ok(MutationStatus::Accepted)
        }

        ConflictOutcome::Accepted { current_payload } => {
            // LWW: incoming wins. For KnowledgeNote, create a conflict copy instead.
            if matches!(envelope.entity_type, EntityType::KnowledgeNote) {
                if let Some(payload) = &envelope.payload {
                    let conflict_id = create_conflict_copy(
                        tx,
                        envelope.entity_id,
                        payload,
                        auth_user.user_id,
                        envelope.mutation_id,
                        envelope.base_version,
                        Some(
                            current_payload
                                .get("updated_at")
                                .and_then(|v| v.as_str())
                                .and_then(|s| s.parse::<DateTime<Utc>>().ok())
                                .unwrap_or(envelope.occurred_at),
                        ),
                    )
                    .await?;
                    return Ok(MutationStatus::Conflicted { conflict_id });
                }
            }

            // TrackingItemEvent + quantity conflict → Conflict error (S009 wiring).
            if matches!(envelope.entity_type, EntityType::TrackingItemEvent) {
                if let Some(payload) = &envelope.payload {
                    if check_quantity_conflict(payload) {
                        return Err(AppError::Conflict(
                            "quantity conflict — re-read and resubmit".to_string(),
                        ));
                    }
                }
            }

            // General LWW: log conflict, apply incoming mutation.
            let conflict_id =
                record_conflict_row(tx, envelope, &current_payload, auth_user.user_id).await?;
            apply_update_to_table(tx, &envelope.entity_type, envelope.entity_id).await?;
            Ok(MutationStatus::Conflicted { conflict_id })
        }

        ConflictOutcome::Rejected { current_payload } => {
            // For KnowledgeNote: still create a conflict copy even when rejected
            // (the spec says "instead of LWW, create a new note row").
            if matches!(envelope.entity_type, EntityType::KnowledgeNote) {
                if let Some(payload) = &envelope.payload {
                    let conflict_id = create_conflict_copy(
                        tx,
                        envelope.entity_id,
                        payload,
                        auth_user.user_id,
                        envelope.mutation_id,
                        envelope.base_version,
                        Some(
                            current_payload
                                .get("updated_at")
                                .and_then(|v| v.as_str())
                                .and_then(|s| s.parse::<DateTime<Utc>>().ok())
                                .unwrap_or(envelope.occurred_at),
                        ),
                    )
                    .await?;
                    return Ok(MutationStatus::Conflicted { conflict_id });
                }
            }

            // TrackingItemEvent + quantity: hard reject (already handled above for append-only path).

            // General LWW reject: log conflict, do NOT apply the update.
            let conflict_id =
                record_conflict_row(tx, envelope, &current_payload, auth_user.user_id).await?;
            Ok(MutationStatus::Conflicted { conflict_id })
        }
    }
}

/// Apply a safe UPDATE (only bumps updated_at) for user-scoped mutable tables.
async fn apply_update_to_table(
    tx: &mut Transaction<'_, Postgres>,
    entity_type: &EntityType,
    entity_id: Uuid,
) -> Result<(), AppError> {
    let table = entity_type_to_table(entity_type);
    let sql = format!("UPDATE {table} SET updated_at = now() WHERE id = $1");
    sqlx::query(&sql)
        .bind(entity_id)
        .execute(tx.as_mut())
        .await
        .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;
    Ok(())
}

/// INSERT a row into sync_conflicts for a general LWW conflict (non-copy path).
async fn record_conflict_row(
    tx: &mut Transaction<'_, Postgres>,
    envelope: &MutationEnvelope,
    current_payload: &serde_json::Value,
    user_id: Uuid,
) -> Result<Uuid, AppError> {
    // Extract current_version from the payload's updated_at field if present.
    let current_version: Option<DateTime<Utc>> = current_payload
        .get("updated_at")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<DateTime<Utc>>().ok());

    let entity_type_str = serde_json::to_value(&envelope.entity_type)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_owned()))
        .unwrap_or_default();

    let row: (Uuid,) = sqlx::query_as(
        "INSERT INTO sync_conflicts \
         (mutation_id, entity_type, entity_id, base_version, current_version, \
          incoming_payload, current_payload, resolution, user_id) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, 'pending', $8) \
         RETURNING id",
    )
    .bind(envelope.mutation_id)
    .bind(&entity_type_str)
    .bind(envelope.entity_id)
    .bind(envelope.base_version)
    .bind(current_version)
    .bind(&envelope.payload)
    .bind(current_payload)
    .bind(user_id)
    .fetch_one(tx.as_mut())
    .await
    .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

    Ok(row.0)
}

/// Handle a Delete mutation.
///
/// Sets `deleted_at = now()` on the target row.
/// TrackingItemEvent and KnowledgeNoteSnapshot have no deleted_at column;
/// for those, delete is not applicable and returns BadRequest.
async fn apply_delete(
    tx: &mut Transaction<'_, Postgres>,
    envelope: &MutationEnvelope,
) -> Result<MutationStatus, AppError> {
    // These tables have no deleted_at column; soft-delete is not applicable.
    // tags: no deleted_at (unique-name constraint would block re-create if hard-deleted).
    // tracking_item_events, knowledge_note_snapshots: append-only / immutable.
    match &envelope.entity_type {
        EntityType::TrackingItemEvent | EntityType::KnowledgeNoteSnapshot | EntityType::Tag => {
            return Err(AppError::BadRequest(format!(
                "{} does not support delete mutations",
                entity_type_to_table(&envelope.entity_type)
            )));
        }
        _ => {}
    }

    let table = entity_type_to_table(&envelope.entity_type);
    let sql = format!("UPDATE {table} SET deleted_at = now() WHERE id = $1");
    sqlx::query(&sql)
        .bind(envelope.entity_id)
        .execute(tx.as_mut())
        .await
        .map_err(|e| AppError::Internal(anyhow::Error::from(e)))?;

    Ok(MutationStatus::Accepted)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::models::AuthUser;
    use crate::contracts::EntityType;
    use crate::sync::models::{MutationEnvelope, MutationStatus, Operation};
    use sqlx::PgPool;
    use uuid::Uuid;

    // ------------------------------------------------------------------
    // Test helpers
    // ------------------------------------------------------------------

    async fn insert_test_user(pool: &PgPool, user_id: Uuid, email: &str) {
        sqlx::query(
            "INSERT INTO users (id, email, display_name, password_hash, is_admin, status) \
             VALUES ($1, $2, 'Test User', 'hashed', false, 'active')",
        )
        .bind(user_id)
        .bind(email)
        .execute(pool)
        .await
        .expect("insert_test_user failed");
    }

    async fn insert_test_household(pool: &PgPool, household_id: Uuid, owner_id: Uuid) {
        sqlx::query(
            "INSERT INTO households (id, owner_id, name) VALUES ($1, $2, 'Test Household')",
        )
        .bind(household_id)
        .bind(owner_id)
        .execute(pool)
        .await
        .expect("insert_test_household failed");
    }

    async fn insert_household_membership(pool: &PgPool, user_id: Uuid, household_id: Uuid) {
        sqlx::query(
            "INSERT INTO household_memberships (household_id, user_id, role) \
             VALUES ($1, $2, 'member')",
        )
        .bind(household_id)
        .bind(user_id)
        .execute(pool)
        .await
        .expect("insert_household_membership failed");
    }

    async fn insert_test_note(
        pool: &PgPool,
        note_id: Uuid,
        user_id: Uuid,
        title: &str,
    ) -> DateTime<Utc> {
        let row: (DateTime<Utc>,) = sqlx::query_as(
            "INSERT INTO knowledge_notes (id, title, user_id) \
             VALUES ($1, $2, $3) RETURNING updated_at",
        )
        .bind(note_id)
        .bind(title)
        .bind(user_id)
        .fetch_one(pool)
        .await
        .expect("insert_test_note failed");
        row.0
    }

    fn make_auth_user(user_id: Uuid) -> AuthUser {
        AuthUser {
            user_id,
            household_ids: vec![],
        }
    }

    fn make_envelope(
        entity_type: EntityType,
        entity_id: Uuid,
        operation: Operation,
        payload: Option<serde_json::Value>,
        base_version: Option<DateTime<Utc>>,
    ) -> MutationEnvelope {
        MutationEnvelope {
            mutation_id: Uuid::new_v4(),
            device_id: Uuid::new_v4(),
            entity_type,
            entity_id,
            operation,
            payload,
            base_version,
            occurred_at: Utc::now(),
        }
    }

    // ------------------------------------------------------------------
    // S005-T: entity_type_to_table
    // ------------------------------------------------------------------

    #[test]
    fn entity_type_to_table_user_scoped() {
        assert_eq!(
            entity_type_to_table(&EntityType::KnowledgeNote),
            "knowledge_notes"
        );
        assert_eq!(entity_type_to_table(&EntityType::Initiative), "initiatives");
        assert_eq!(
            entity_type_to_table(&EntityType::GuidanceEpic),
            "guidance_epics"
        );
        assert_eq!(entity_type_to_table(&EntityType::Tag), "tags");
    }

    #[test]
    fn entity_type_to_table_household_scoped() {
        assert_eq!(
            entity_type_to_table(&EntityType::TrackingLocation),
            "tracking_locations"
        );
        assert_eq!(
            entity_type_to_table(&EntityType::TrackingCategory),
            "tracking_categories"
        );
        assert_eq!(
            entity_type_to_table(&EntityType::TrackingShoppingList),
            "tracking_shopping_lists"
        );
        assert_eq!(
            entity_type_to_table(&EntityType::TrackingItemEvent),
            "tracking_item_events"
        );
    }

    #[test]
    fn entity_type_to_table_all_variants_covered() {
        // Exhaustive check: ensure every variant returns a non-empty string.
        let all = [
            EntityType::User,
            EntityType::Household,
            EntityType::Initiative,
            EntityType::Tag,
            EntityType::Attachment,
            EntityType::GuidanceEpic,
            EntityType::GuidanceQuest,
            EntityType::GuidanceRoutine,
            EntityType::GuidanceFocusSession,
            EntityType::GuidanceDailyCheckin,
            EntityType::KnowledgeNote,
            EntityType::KnowledgeNoteSnapshot,
            EntityType::TrackingLocation,
            EntityType::TrackingCategory,
            EntityType::TrackingItem,
            EntityType::TrackingItemEvent,
            EntityType::TrackingShoppingList,
            EntityType::TrackingShoppingListItem,
        ];
        for et in all {
            let table = entity_type_to_table(&et);
            assert!(
                !table.is_empty(),
                "entity_type_to_table returned empty for {:?}",
                et
            );
        }
    }

    // ------------------------------------------------------------------
    // S005-T: check_ownership (DB-backed)
    // ------------------------------------------------------------------

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn ownership_ok_for_valid_owner(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "owner@example.com").await;
        let note_id = Uuid::new_v4();
        insert_test_note(&pool, note_id, user_id, "My Note").await;

        let auth = make_auth_user(user_id);
        let result = check_ownership(&pool, &EntityType::KnowledgeNote, note_id, &auth).await;
        assert!(
            result.is_ok(),
            "owner should have access; got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn ownership_forbidden_for_non_owner(pool: PgPool) {
        let owner = Uuid::new_v4();
        let other = Uuid::new_v4();
        insert_test_user(&pool, owner, "owner2@example.com").await;
        insert_test_user(&pool, other, "other@example.com").await;
        let note_id = Uuid::new_v4();
        insert_test_note(&pool, note_id, owner, "Not Your Note").await;

        let auth = make_auth_user(other);
        let result = check_ownership(&pool, &EntityType::KnowledgeNote, note_id, &auth).await;
        assert!(
            matches!(result, Err(AppError::Forbidden)),
            "non-owner should get Forbidden; got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn ownership_forbidden_for_unknown_entity_id(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "unknown@example.com").await;

        let auth = make_auth_user(user_id);
        let result =
            check_ownership(&pool, &EntityType::KnowledgeNote, Uuid::new_v4(), &auth).await;
        assert!(
            matches!(result, Err(AppError::Forbidden)),
            "unknown entity_id should get Forbidden; got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn ownership_ok_for_household_member(pool: PgPool) {
        let owner = Uuid::new_v4();
        let member = Uuid::new_v4();
        insert_test_user(&pool, owner, "hh_owner@example.com").await;
        insert_test_user(&pool, member, "hh_member@example.com").await;
        let hh_id = Uuid::new_v4();
        insert_test_household(&pool, hh_id, owner).await;
        insert_household_membership(&pool, member, hh_id).await;

        // Insert a tracking_location for this household.
        let loc_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO tracking_locations (id, name, household_id) VALUES ($1, 'Pantry', $2)",
        )
        .bind(loc_id)
        .bind(hh_id)
        .execute(&pool)
        .await
        .expect("insert tracking_location failed");

        let auth = make_auth_user(member);
        let result = check_ownership(&pool, &EntityType::TrackingLocation, loc_id, &auth).await;
        assert!(
            result.is_ok(),
            "household member should have access; got: {:?}",
            result
        );
    }

    // ------------------------------------------------------------------
    // S009-T: check_quantity_conflict
    // ------------------------------------------------------------------

    #[test]
    fn quantity_conflict_true_when_quantity_present() {
        let payload = serde_json::json!({ "quantity": 5 });
        assert!(check_quantity_conflict(&payload));
    }

    #[test]
    fn quantity_conflict_true_when_consumed_quantity_present() {
        let payload = serde_json::json!({ "consumed_quantity": 2, "note": "some text" });
        assert!(check_quantity_conflict(&payload));
    }

    #[test]
    fn quantity_conflict_false_when_no_quantity_keys() {
        let payload = serde_json::json!({ "note": "no qty here", "event_type": "use" });
        assert!(!check_quantity_conflict(&payload));
    }

    #[test]
    fn quantity_conflict_false_for_null_payload() {
        assert!(!check_quantity_conflict(&serde_json::Value::Null));
    }

    // ------------------------------------------------------------------
    // S006-T: apply_mutations — create → accepted + sync_mutations row
    // ------------------------------------------------------------------

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn valid_create_mutation_accepted_and_recorded(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "create_test@example.com").await;

        let note_id = Uuid::new_v4();
        let envelope = make_envelope(
            EntityType::KnowledgeNote,
            note_id,
            Operation::Create,
            Some(serde_json::json!({ "title": "New Note", "content": "Hello" })),
            None,
        );
        let mutation_id = envelope.mutation_id;
        let auth = make_auth_user(user_id);

        let results = apply_mutations(&pool, vec![envelope], auth)
            .await
            .expect("apply_mutations failed");

        assert_eq!(results.len(), 1);
        assert!(
            matches!(results[0].status, MutationStatus::Accepted),
            "status should be Accepted; got: {:?}",
            results[0].status
        );

        // Verify sync_mutations row was written.
        let recorded: Option<(Uuid,)> =
            sqlx::query_as("SELECT mutation_id FROM sync_mutations WHERE mutation_id = $1")
                .bind(mutation_id)
                .fetch_optional(&pool)
                .await
                .expect("query failed");
        assert!(
            recorded.is_some(),
            "sync_mutations row must exist after accepted mutation"
        );

        // Verify the note was created.
        let note_exists: Option<(Uuid,)> =
            sqlx::query_as("SELECT id FROM knowledge_notes WHERE id = $1")
                .bind(note_id)
                .fetch_optional(&pool)
                .await
                .expect("query failed");
        assert!(note_exists.is_some(), "knowledge_notes row must exist");
    }

    // S006-T: replaying same mutation_id → deduplicated + no duplicate DB row
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn replaying_same_mutation_id_returns_deduplicated(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "dedup_test@example.com").await;

        let note_id = Uuid::new_v4();
        let envelope = make_envelope(
            EntityType::KnowledgeNote,
            note_id,
            Operation::Create,
            Some(serde_json::json!({ "title": "Dedup Note" })),
            None,
        );
        let auth = make_auth_user(user_id);

        // First application — accepted.
        let first = apply_mutations(&pool, vec![envelope.clone()], auth.clone())
            .await
            .expect("first apply_mutations failed");
        assert!(matches!(first[0].status, MutationStatus::Accepted));

        // Second application — deduplicated.
        let second = apply_mutations(&pool, vec![envelope], auth)
            .await
            .expect("second apply_mutations failed");
        assert!(
            matches!(second[0].status, MutationStatus::Deduplicated),
            "second submission should be Deduplicated; got: {:?}",
            second[0].status
        );

        // Verify no duplicate notes.
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM knowledge_notes WHERE id = $1")
            .bind(note_id)
            .fetch_one(&pool)
            .await
            .expect("count query failed");
        assert_eq!(count.0, 1, "exactly 1 note row must exist after dedup");
    }

    // S006-T: mutation for entity owned by other user → Forbidden
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn mutation_for_other_users_entity_returns_forbidden(pool: PgPool) {
        let owner = Uuid::new_v4();
        let attacker = Uuid::new_v4();
        insert_test_user(&pool, owner, "real_owner@example.com").await;
        insert_test_user(&pool, attacker, "attacker@example.com").await;

        let note_id = Uuid::new_v4();
        insert_test_note(&pool, note_id, owner, "Owner's Private Note").await;

        let envelope = make_envelope(
            EntityType::KnowledgeNote,
            note_id,
            Operation::Update,
            Some(serde_json::json!({ "title": "Hacked" })),
            None,
        );
        let auth = make_auth_user(attacker);

        let result = apply_mutations(&pool, vec![envelope], auth).await;
        assert!(
            matches!(result, Err(AppError::Forbidden)),
            "attacker must get Forbidden; got: {:?}",
            result
        );
    }

    // S006-T: delete mutation sets deleted_at, row remains (FA-009)
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn delete_mutation_sets_deleted_at_row_remains(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "delete_mut@example.com").await;

        let note_id = Uuid::new_v4();
        insert_test_note(&pool, note_id, user_id, "To Be Deleted").await;

        let envelope = make_envelope(
            EntityType::KnowledgeNote,
            note_id,
            Operation::Delete,
            None,
            None,
        );
        let auth = make_auth_user(user_id);

        let results = apply_mutations(&pool, vec![envelope], auth)
            .await
            .expect("apply_mutations failed");
        assert!(matches!(results[0].status, MutationStatus::Accepted));

        // Verify deleted_at IS NOT NULL and row still exists.
        let row: Option<(Option<DateTime<Utc>>,)> =
            sqlx::query_as("SELECT deleted_at FROM knowledge_notes WHERE id = $1")
                .bind(note_id)
                .fetch_optional(&pool)
                .await
                .expect("query failed");

        assert!(row.is_some(), "row must still exist after soft delete");
        assert!(
            row.unwrap().0.is_some(),
            "deleted_at must be non-null after delete mutation"
        );
    }

    // ------------------------------------------------------------------
    // S007-T: detect_conflict
    // ------------------------------------------------------------------

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn no_conflict_when_base_version_matches(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "conflict_clean@example.com").await;
        let note_id = Uuid::new_v4();
        let updated_at = insert_test_note(&pool, note_id, user_id, "Clean Note").await;

        let mut tx = pool.begin().await.expect("begin tx");
        let outcome = detect_conflict(
            &mut tx,
            &EntityType::KnowledgeNote,
            note_id,
            Some(updated_at),
            Utc::now(),
        )
        .await
        .expect("detect_conflict failed");
        tx.rollback().await.ok();

        assert!(matches!(outcome, ConflictOutcome::Clean), "should be Clean");
    }

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn conflict_with_newer_occurred_at_returns_accepted(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "conflict_accept@example.com").await;
        let note_id = Uuid::new_v4();
        let updated_at = insert_test_note(&pool, note_id, user_id, "Accept Note").await;

        // Simulate another device updating the note after our base_version.
        sqlx::query(
            "UPDATE knowledge_notes SET updated_at = now() + interval '10 seconds' WHERE id = $1",
        )
        .bind(note_id)
        .execute(&pool)
        .await
        .expect("simulated update failed");

        // Our occurred_at is very recent — newer than the simulated server update.
        let occurred_at = Utc::now() + chrono::Duration::minutes(1);

        let mut tx = pool.begin().await.expect("begin tx");
        let outcome = detect_conflict(
            &mut tx,
            &EntityType::KnowledgeNote,
            note_id,
            Some(updated_at),
            occurred_at,
        )
        .await
        .expect("detect_conflict failed");
        tx.rollback().await.ok();

        assert!(
            matches!(outcome, ConflictOutcome::Accepted { .. }),
            "should be Accepted (LWW: incoming wins); got other variant"
        );
    }

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn conflict_with_older_occurred_at_returns_rejected(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "conflict_reject@example.com").await;
        let note_id = Uuid::new_v4();
        let updated_at = insert_test_note(&pool, note_id, user_id, "Reject Note").await;

        // Simulate server update with a timestamp far in the future.
        sqlx::query(
            "UPDATE knowledge_notes SET updated_at = now() + interval '1 hour' WHERE id = $1",
        )
        .bind(note_id)
        .execute(&pool)
        .await
        .expect("simulated update failed");

        // Our occurred_at is older than updated_at (in the past relative to the simulated update).
        let occurred_at = updated_at - chrono::Duration::seconds(5);

        let mut tx = pool.begin().await.expect("begin tx");
        let outcome = detect_conflict(
            &mut tx,
            &EntityType::KnowledgeNote,
            note_id,
            Some(updated_at),
            occurred_at,
        )
        .await
        .expect("detect_conflict failed");
        tx.rollback().await.ok();

        assert!(
            matches!(outcome, ConflictOutcome::Rejected { .. }),
            "should be Rejected (LWW: existing wins); got other variant"
        );
    }

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn null_base_version_on_non_create_uses_lww(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "null_base@example.com").await;
        let note_id = Uuid::new_v4();
        insert_test_note(&pool, note_id, user_id, "LWW Note").await;

        // occurred_at is in the future → should get Accepted (incoming wins).
        let occurred_at = Utc::now() + chrono::Duration::hours(1);

        let mut tx = pool.begin().await.expect("begin tx");
        let outcome = detect_conflict(
            &mut tx,
            &EntityType::KnowledgeNote,
            note_id,
            None, // null base_version
            occurred_at,
        )
        .await
        .expect("detect_conflict failed");
        tx.rollback().await.ok();

        assert!(
            matches!(outcome, ConflictOutcome::Accepted { .. }),
            "null base_version + future occurred_at should yield Accepted"
        );
    }

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn sync_conflicts_row_written_on_conflict(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "conflict_row@example.com").await;
        let note_id = Uuid::new_v4();
        let old_updated_at = insert_test_note(&pool, note_id, user_id, "Conflict Row Note").await;

        // Simulate a concurrent update on the server side.
        sqlx::query(
            "UPDATE knowledge_notes SET updated_at = now() + interval '5 seconds' WHERE id = $1",
        )
        .bind(note_id)
        .execute(&pool)
        .await
        .expect("simulated update failed");

        // Submit a conflicting update with an occurred_at far in the future (LWW win).
        let mutation_id = Uuid::new_v4();
        let envelope = MutationEnvelope {
            mutation_id,
            device_id: Uuid::new_v4(),
            entity_type: EntityType::KnowledgeNote,
            entity_id: note_id,
            operation: Operation::Update,
            payload: Some(serde_json::json!({ "title": "Conflicting update" })),
            base_version: Some(old_updated_at),
            occurred_at: Utc::now() + chrono::Duration::hours(1),
        };
        let auth = make_auth_user(user_id);

        let results = apply_mutations(&pool, vec![envelope], auth)
            .await
            .expect("apply_mutations failed");

        // For KnowledgeNote + conflict → Conflicted status with a conflict copy.
        assert!(
            matches!(results[0].status, MutationStatus::Conflicted { .. }),
            "expected Conflicted; got: {:?}",
            results[0].status
        );

        // Verify sync_conflicts row was written.
        let conflict_count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM sync_conflicts WHERE entity_id = $1")
                .bind(note_id)
                .fetch_one(&pool)
                .await
                .expect("count query failed");
        assert!(
            conflict_count.0 > 0,
            "sync_conflicts row must exist after conflict"
        );
    }

    // ------------------------------------------------------------------
    // S008-T: create_conflict_copy (FA-007)
    // ------------------------------------------------------------------

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn conflict_copy_creates_two_notes(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "copy_test@example.com").await;
        let note_id = Uuid::new_v4();
        let old_updated_at = insert_test_note(&pool, note_id, user_id, "Original Note").await;

        // Simulate a concurrent update on the server side.
        sqlx::query(
            "UPDATE knowledge_notes SET updated_at = now() + interval '5 seconds' WHERE id = $1",
        )
        .bind(note_id)
        .execute(&pool)
        .await
        .expect("simulated update failed");

        let envelope = MutationEnvelope {
            mutation_id: Uuid::new_v4(),
            device_id: Uuid::new_v4(),
            entity_type: EntityType::KnowledgeNote,
            entity_id: note_id,
            operation: Operation::Update,
            payload: Some(serde_json::json!({ "title": "My Conflicting Version" })),
            base_version: Some(old_updated_at),
            occurred_at: Utc::now() + chrono::Duration::hours(1),
        };
        let auth = make_auth_user(user_id);

        let results = apply_mutations(&pool, vec![envelope], auth)
            .await
            .expect("apply_mutations failed");
        assert!(matches!(
            results[0].status,
            MutationStatus::Conflicted { .. }
        ));

        // FA-007: two knowledge_notes rows (original + conflict copy).
        let count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM knowledge_notes WHERE user_id = $1")
                .bind(user_id)
                .fetch_one(&pool)
                .await
                .expect("count notes failed");
        assert_eq!(
            count.0, 2,
            "expected 2 knowledge_notes rows; got {}",
            count.0
        );
    }

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn conflict_copy_creates_entity_relation_duplicates(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "copy_rel@example.com").await;
        let note_id = Uuid::new_v4();
        let old_updated_at =
            insert_test_note(&pool, note_id, user_id, "Original Relation Note").await;

        sqlx::query(
            "UPDATE knowledge_notes SET updated_at = now() + interval '5 seconds' WHERE id = $1",
        )
        .bind(note_id)
        .execute(&pool)
        .await
        .expect("simulated update failed");

        let envelope = MutationEnvelope {
            mutation_id: Uuid::new_v4(),
            device_id: Uuid::new_v4(),
            entity_type: EntityType::KnowledgeNote,
            entity_id: note_id,
            operation: Operation::Update,
            payload: Some(serde_json::json!({ "title": "Conflict Copy Relation" })),
            base_version: Some(old_updated_at),
            occurred_at: Utc::now() + chrono::Duration::hours(1),
        };
        let auth = make_auth_user(user_id);

        apply_mutations(&pool, vec![envelope], auth)
            .await
            .expect("apply_mutations failed");

        // Verify entity_relations row with relation_type = "duplicates".
        let rel: Option<(String,)> = sqlx::query_as(
            "SELECT relation_type FROM entity_relations \
             WHERE to_entity_id = $1 AND relation_type = 'duplicates'",
        )
        .bind(note_id)
        .fetch_optional(&pool)
        .await
        .expect("query failed");

        assert!(
            rel.is_some(),
            "entity_relations 'duplicates' row must exist after conflict copy"
        );
    }

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn conflict_copy_writes_sync_conflicts_row_with_conflict_copy_resolution(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "copy_conflict@example.com").await;
        let note_id = Uuid::new_v4();
        let old_updated_at =
            insert_test_note(&pool, note_id, user_id, "Conflict Resolution Note").await;

        sqlx::query(
            "UPDATE knowledge_notes SET updated_at = now() + interval '5 seconds' WHERE id = $1",
        )
        .bind(note_id)
        .execute(&pool)
        .await
        .expect("simulated update failed");

        let envelope = MutationEnvelope {
            mutation_id: Uuid::new_v4(),
            device_id: Uuid::new_v4(),
            entity_type: EntityType::KnowledgeNote,
            entity_id: note_id,
            operation: Operation::Update,
            payload: Some(serde_json::json!({ "title": "Conflict Resolution" })),
            base_version: Some(old_updated_at),
            occurred_at: Utc::now() + chrono::Duration::hours(1),
        };
        let auth = make_auth_user(user_id);

        apply_mutations(&pool, vec![envelope], auth)
            .await
            .expect("apply_mutations failed");

        // Verify sync_conflicts has resolution = "conflict_copy".
        let resolution: Option<(String,)> =
            sqlx::query_as("SELECT resolution FROM sync_conflicts WHERE entity_id = $1")
                .bind(note_id)
                .fetch_optional(&pool)
                .await
                .expect("query failed");

        assert_eq!(
            resolution.map(|r| r.0),
            Some("conflict_copy".to_string()),
            "sync_conflicts resolution must be 'conflict_copy'"
        );
    }

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn conflict_copy_original_note_unchanged(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "copy_unchanged@example.com").await;
        let note_id = Uuid::new_v4();
        let old_updated_at = insert_test_note(&pool, note_id, user_id, "Unchanged Original").await;

        sqlx::query(
            "UPDATE knowledge_notes SET updated_at = now() + interval '5 seconds' WHERE id = $1",
        )
        .bind(note_id)
        .execute(&pool)
        .await
        .expect("simulated update failed");

        let envelope = MutationEnvelope {
            mutation_id: Uuid::new_v4(),
            device_id: Uuid::new_v4(),
            entity_type: EntityType::KnowledgeNote,
            entity_id: note_id,
            operation: Operation::Update,
            payload: Some(serde_json::json!({ "title": "Conflict" })),
            base_version: Some(old_updated_at),
            occurred_at: Utc::now() + chrono::Duration::hours(1),
        };
        let auth = make_auth_user(user_id);

        apply_mutations(&pool, vec![envelope], auth)
            .await
            .expect("apply_mutations failed");

        // Original note title must remain "Unchanged Original".
        let title: (String,) = sqlx::query_as("SELECT title FROM knowledge_notes WHERE id = $1")
            .bind(note_id)
            .fetch_one(&pool)
            .await
            .expect("fetch original note failed");

        assert_eq!(
            title.0, "Unchanged Original",
            "original note must not be modified by conflict copy"
        );
    }

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn conflict_copy_result_is_conflicted_status(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "copy_status@example.com").await;
        let note_id = Uuid::new_v4();
        let old_updated_at = insert_test_note(&pool, note_id, user_id, "Status Note").await;

        sqlx::query(
            "UPDATE knowledge_notes SET updated_at = now() + interval '5 seconds' WHERE id = $1",
        )
        .bind(note_id)
        .execute(&pool)
        .await
        .expect("simulated update failed");

        let envelope = MutationEnvelope {
            mutation_id: Uuid::new_v4(),
            device_id: Uuid::new_v4(),
            entity_type: EntityType::KnowledgeNote,
            entity_id: note_id,
            operation: Operation::Update,
            payload: Some(serde_json::json!({ "title": "My Version" })),
            base_version: Some(old_updated_at),
            occurred_at: Utc::now() + chrono::Duration::hours(1),
        };
        let auth = make_auth_user(user_id);

        let results = apply_mutations(&pool, vec![envelope], auth)
            .await
            .expect("apply_mutations failed");

        assert!(
            matches!(results[0].status, MutationStatus::Conflicted { .. }),
            "expected Conflicted {{ conflict_id }}, got: {:?}",
            results[0].status
        );
    }

    // ------------------------------------------------------------------
    // S009-T: quantity conflict (FA-008)
    // ------------------------------------------------------------------

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn tracking_item_event_update_with_quantity_returns_conflict_error(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "qty_conflict@example.com").await;
        let hh_id = Uuid::new_v4();
        insert_test_household(&pool, hh_id, user_id).await;

        // Insert a tracking_item.
        let item_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO tracking_items (id, name, user_id, household_id) VALUES ($1, 'Salt', $2, $3)",
        )
        .bind(item_id)
        .bind(user_id)
        .bind(hh_id)
        .execute(&pool)
        .await
        .expect("insert tracking_item failed");

        // Insert a tracking_item_event.
        let event_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO tracking_item_events \
             (id, item_id, event_type, quantity_change, occurred_at) \
             VALUES ($1, $2, 'use', 1, now())",
        )
        .bind(event_id)
        .bind(item_id)
        .execute(&pool)
        .await
        .expect("insert tracking_item_event failed");

        // Update with quantity field → should return Conflict error.
        let envelope = make_envelope(
            EntityType::TrackingItemEvent,
            event_id,
            Operation::Update,
            Some(serde_json::json!({ "quantity": 5 })),
            None,
        );
        let auth = make_auth_user(user_id);

        let result = apply_mutations(&pool, vec![envelope], auth).await;
        assert!(
            matches!(result, Err(AppError::Conflict(_))),
            "quantity conflict must return Conflict error; got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn tracking_item_event_update_without_quantity_accepted(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "qty_no_conflict@example.com").await;
        let hh_id = Uuid::new_v4();
        insert_test_household(&pool, hh_id, user_id).await;

        let item_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO tracking_items (id, name, user_id, household_id) VALUES ($1, 'Pepper', $2, $3)",
        )
        .bind(item_id)
        .bind(user_id)
        .bind(hh_id)
        .execute(&pool)
        .await
        .expect("insert tracking_item failed");

        let event_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO tracking_item_events \
             (id, item_id, event_type, quantity_change, occurred_at) \
             VALUES ($1, $2, 'note', 0, now())",
        )
        .bind(event_id)
        .bind(item_id)
        .execute(&pool)
        .await
        .expect("insert tracking_item_event failed");

        // Update without quantity field → should be accepted.
        let envelope = make_envelope(
            EntityType::TrackingItemEvent,
            event_id,
            Operation::Update,
            Some(serde_json::json!({ "notes": "updated note text" })),
            None,
        );
        let auth = make_auth_user(user_id);

        let results = apply_mutations(&pool, vec![envelope], auth)
            .await
            .expect("apply_mutations failed");
        assert!(
            matches!(results[0].status, MutationStatus::Accepted),
            "non-quantity update should be Accepted; got: {:?}",
            results[0].status
        );
    }

    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn non_tracking_entity_with_conflict_does_not_use_quantity_check(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "non_tracking@example.com").await;
        let note_id = Uuid::new_v4();
        let old_updated_at = insert_test_note(&pool, note_id, user_id, "Non-tracking").await;

        sqlx::query(
            "UPDATE knowledge_notes SET updated_at = now() + interval '5 seconds' WHERE id = $1",
        )
        .bind(note_id)
        .execute(&pool)
        .await
        .expect("simulated update failed");

        // KnowledgeNote update with "quantity" in payload — should NOT trigger quantity conflict.
        // Instead, it should go through the conflict copy path.
        let envelope = MutationEnvelope {
            mutation_id: Uuid::new_v4(),
            device_id: Uuid::new_v4(),
            entity_type: EntityType::KnowledgeNote,
            entity_id: note_id,
            operation: Operation::Update,
            payload: Some(serde_json::json!({ "title": "Note about quantity", "quantity": 3 })),
            base_version: Some(old_updated_at),
            occurred_at: Utc::now() + chrono::Duration::hours(1),
        };
        let auth = make_auth_user(user_id);

        let result = apply_mutations(&pool, vec![envelope], auth).await;
        // Should NOT be Conflict error (409); should be Conflicted status (conflict copy).
        assert!(
            result.is_ok(),
            "KnowledgeNote with quantity payload must not return Conflict error"
        );
        let results = result.unwrap();
        assert!(
            matches!(results[0].status, MutationStatus::Conflicted { .. }),
            "should be Conflicted (conflict copy), not quantity error; got: {:?}",
            results[0].status
        );
    }
}
