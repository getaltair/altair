use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use super::models::{
    CreateItemEventRequest, CreateItemRequest, TrackingItem, TrackingItemEvent, UpdateItemRequest,
};

/// Create a new tracking item in the given household
pub async fn create_item(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateItemRequest,
) -> Result<TrackingItem, AppError> {
    let quantity = req.quantity.unwrap_or(0);

    sqlx::query_as::<_, TrackingItem>(
        r#"INSERT INTO tracking_items
               (user_id, household_id, category_id, location_id, name, description,
                quantity, unit, min_quantity, barcode)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
           RETURNING id, user_id, household_id, category_id, location_id, name,
                     description, quantity, unit, min_quantity, barcode, status,
                     created_at, updated_at"#,
    )
    .bind(user_id)
    .bind(req.household_id)
    .bind(req.category_id)
    .bind(req.location_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(quantity)
    .bind(&req.unit)
    .bind(req.min_quantity)
    .bind(&req.barcode)
    .fetch_one(pool)
    .await
    .map_err(AppError::Database)
}

/// List all tracking items belonging to a household
pub async fn list_items(
    pool: &PgPool,
    household_id: Uuid,
) -> Result<Vec<TrackingItem>, AppError> {
    sqlx::query_as::<_, TrackingItem>(
        r#"SELECT id, user_id, household_id, category_id, location_id, name,
                  description, quantity, unit, min_quantity, barcode, status,
                  created_at, updated_at
           FROM tracking_items
           WHERE household_id = $1
           ORDER BY created_at DESC"#,
    )
    .bind(household_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}

/// Get a single tracking item by ID
pub async fn get_item(
    pool: &PgPool,
    id: Uuid,
) -> Result<TrackingItem, AppError> {
    sqlx::query_as::<_, TrackingItem>(
        r#"SELECT id, user_id, household_id, category_id, location_id, name,
                  description, quantity, unit, min_quantity, barcode, status,
                  created_at, updated_at
           FROM tracking_items
           WHERE id = $1"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound("Tracking item not found".to_string()))
}

/// Update an existing tracking item with partial-update semantics
pub async fn update_item(
    pool: &PgPool,
    id: Uuid,
    req: UpdateItemRequest,
) -> Result<TrackingItem, AppError> {
    let mut qb: sqlx::QueryBuilder<sqlx::Postgres> =
        sqlx::QueryBuilder::new("UPDATE tracking_items SET updated_at = now()");

    if let Some(ref name) = req.name {
        qb.push(", name = ");
        qb.push_bind(name.clone());
    }

    if let Some(ref description) = req.description {
        qb.push(", description = ");
        qb.push_bind(description.clone());
    }

    if let Some(ref category_id) = req.category_id {
        qb.push(", category_id = ");
        qb.push_bind(*category_id);
    }

    if let Some(ref location_id) = req.location_id {
        qb.push(", location_id = ");
        qb.push_bind(*location_id);
    }

    if let Some(quantity) = req.quantity {
        qb.push(", quantity = ");
        qb.push_bind(quantity);
    }

    if let Some(ref unit) = req.unit {
        qb.push(", unit = ");
        qb.push_bind(unit.clone());
    }

    if let Some(min_quantity) = req.min_quantity {
        qb.push(", min_quantity = ");
        qb.push_bind(min_quantity);
    }

    if let Some(ref barcode) = req.barcode {
        qb.push(", barcode = ");
        qb.push_bind(barcode.clone());
    }

    if let Some(ref status) = req.status {
        qb.push(", status = ");
        qb.push_bind(status.clone());
    }

    qb.push(" WHERE id = ");
    qb.push_bind(id);
    qb.push(
        " RETURNING id, user_id, household_id, category_id, location_id, name, \
         description, quantity, unit, min_quantity, barcode, status, created_at, updated_at",
    );

    qb.build_query_as::<TrackingItem>()
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("Tracking item not found".to_string()))
}

/// Delete a tracking item by ID
pub async fn delete_item(
    pool: &PgPool,
    id: Uuid,
) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM tracking_items WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Tracking item not found".to_string()));
    }

    Ok(())
}

/// Create an item event and atomically update the item's quantity within a transaction
pub async fn create_item_event(
    pool: &PgPool,
    item_id: Uuid,
    user_id: Uuid,
    req: CreateItemEventRequest,
) -> Result<TrackingItemEvent, AppError> {
    let mut tx = pool.begin().await.map_err(AppError::Database)?;

    let event = sqlx::query_as::<_, TrackingItemEvent>(
        r#"INSERT INTO tracking_item_events (item_id, user_id, event_type, quantity_change, notes)
           VALUES ($1, $2, $3, $4, $5)
           RETURNING id, item_id, user_id, event_type, quantity_change, notes, created_at"#,
    )
    .bind(item_id)
    .bind(user_id)
    .bind(&req.event_type)
    .bind(req.quantity_change)
    .bind(&req.notes)
    .fetch_one(&mut *tx)
    .await
    .map_err(AppError::Database)?;

    sqlx::query(
        "UPDATE tracking_items SET quantity = quantity + $1, updated_at = now() WHERE id = $2",
    )
    .bind(req.quantity_change)
    .bind(item_id)
    .execute(&mut *tx)
    .await
    .map_err(AppError::Database)?;

    tx.commit().await.map_err(AppError::Database)?;

    Ok(event)
}

/// List all events for a given item, ordered by most recent first
pub async fn list_item_events(
    pool: &PgPool,
    item_id: Uuid,
) -> Result<Vec<TrackingItemEvent>, AppError> {
    sqlx::query_as::<_, TrackingItemEvent>(
        r#"SELECT id, item_id, user_id, event_type, quantity_change, notes, created_at
           FROM tracking_item_events
           WHERE item_id = $1
           ORDER BY created_at DESC"#,
    )
    .bind(item_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}

/// List items that are below their minimum quantity threshold
pub async fn list_low_stock_items(
    pool: &PgPool,
    household_id: Uuid,
) -> Result<Vec<TrackingItem>, AppError> {
    sqlx::query_as::<_, TrackingItem>(
        r#"SELECT id, user_id, household_id, category_id, location_id, name,
                  description, quantity, unit, min_quantity, barcode, status,
                  created_at, updated_at
           FROM tracking_items
           WHERE household_id = $1 AND min_quantity IS NOT NULL AND quantity < min_quantity"#,
    )
    .bind(household_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}
