use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::error::AppError;
use super::models::{
    CreateShoppingListItemRequest, CreateShoppingListRequest, TrackingShoppingList,
    TrackingShoppingListItem, UpdateShoppingListItemRequest, UpdateShoppingListRequest,
};

/// Create a new shopping list for the given user and household.
///
/// The caller is responsible for verifying household membership before
/// invoking this function.
pub async fn create_list(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateShoppingListRequest,
) -> Result<TrackingShoppingList, AppError> {
    sqlx::query_as::<_, TrackingShoppingList>(
        r#"INSERT INTO tracking_shopping_lists (user_id, household_id, name)
           VALUES ($1, $2, $3)
           RETURNING id, user_id, household_id, name, status, created_at, updated_at"#,
    )
    .bind(user_id)
    .bind(req.household_id)
    .bind(&req.name)
    .fetch_one(pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
            AppError::BadRequest("Invalid household".to_string())
        }
        _ => AppError::Database(e),
    })
}

/// List all shopping lists for a household.
///
/// The caller is responsible for verifying household membership before
/// invoking this function.
pub async fn list_lists(
    pool: &PgPool,
    household_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<TrackingShoppingList>, AppError> {
    sqlx::query_as::<_, TrackingShoppingList>(
        r#"SELECT id, user_id, household_id, name, status, created_at, updated_at
           FROM tracking_shopping_lists
           WHERE household_id = $1
           ORDER BY created_at DESC
           LIMIT $2 OFFSET $3"#,
    )
    .bind(household_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}

/// Get a single shopping list by ID
pub async fn get_list(
    pool: &PgPool,
    id: Uuid,
) -> Result<TrackingShoppingList, AppError> {
    sqlx::query_as::<_, TrackingShoppingList>(
        r#"SELECT id, user_id, household_id, name, status, created_at, updated_at
           FROM tracking_shopping_lists
           WHERE id = $1"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound("Shopping list not found".to_string()))
}

/// Update an existing shopping list (partial update).
///
/// Only fields present in the request are modified. The `status` field
/// uses the `ShoppingListStatus` enum for type-safe validation.
///
/// The caller is responsible for verifying household membership before
/// invoking this function.
pub async fn update_list(
    pool: &PgPool,
    id: Uuid,
    req: UpdateShoppingListRequest,
) -> Result<TrackingShoppingList, AppError> {
    let mut qb: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new("UPDATE tracking_shopping_lists SET updated_at = now()");

    if let Some(ref name) = req.name {
        qb.push(", name = ");
        qb.push_bind(name.clone());
    }

    if let Some(ref status) = req.status {
        qb.push(", status = ");
        qb.push_bind(status.as_str());
    }

    qb.push(" WHERE id = ");
    qb.push_bind(id);
    qb.push(" RETURNING id, user_id, household_id, name, status, created_at, updated_at");

    qb.build_query_as::<TrackingShoppingList>()
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("Shopping list not found".to_string()))
}

/// Delete a shopping list by ID.
///
/// The caller is responsible for verifying household membership before
/// invoking this function.
pub async fn delete_list(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM tracking_shopping_lists WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Shopping list not found".to_string()));
    }

    Ok(())
}

/// Add an item to a shopping list.
///
/// The caller is responsible for verifying household membership before
/// invoking this function.
pub async fn add_list_item(
    pool: &PgPool,
    shopping_list_id: Uuid,
    req: CreateShoppingListItemRequest,
) -> Result<TrackingShoppingListItem, AppError> {
    let quantity = req.quantity.unwrap_or(1);

    sqlx::query_as::<_, TrackingShoppingListItem>(
        r#"INSERT INTO tracking_shopping_list_items (shopping_list_id, item_id, name, quantity, unit)
           VALUES ($1, $2, $3, $4, $5)
           RETURNING id, shopping_list_id, item_id, name, quantity, unit, is_checked, created_at"#,
    )
    .bind(shopping_list_id)
    .bind(req.item_id)
    .bind(&req.name)
    .bind(quantity)
    .bind(&req.unit)
    .fetch_one(pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
            AppError::BadRequest("Invalid shopping list or item reference".to_string())
        }
        _ => AppError::Database(e),
    })
}

/// List all items in a shopping list
pub async fn list_list_items(
    pool: &PgPool,
    shopping_list_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<TrackingShoppingListItem>, AppError> {
    sqlx::query_as::<_, TrackingShoppingListItem>(
        r#"SELECT id, shopping_list_id, item_id, name, quantity, unit, is_checked, created_at
           FROM tracking_shopping_list_items
           WHERE shopping_list_id = $1
           ORDER BY created_at ASC
           LIMIT $2 OFFSET $3"#,
    )
    .bind(shopping_list_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}

/// Update a shopping list item (partial update).
///
/// Only fields present in the request are modified. The `unit` field uses
/// double-option semantics: `None` leaves it unchanged, `Some(None)` sets
/// it to NULL, `Some(Some(val))` sets it to `val`.
///
/// The caller is responsible for verifying household membership before
/// invoking this function.
pub async fn update_list_item(
    pool: &PgPool,
    shopping_list_id: Uuid,
    id: Uuid,
    req: UpdateShoppingListItemRequest,
) -> Result<TrackingShoppingListItem, AppError> {
    // No updated_at column on this table; use no-op SET to anchor the dynamic column list
    let mut qb: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new("UPDATE tracking_shopping_list_items SET id = id");

    if let Some(ref name) = req.name {
        qb.push(", name = ");
        qb.push_bind(name.clone());
    }

    if let Some(quantity) = req.quantity {
        qb.push(", quantity = ");
        qb.push_bind(quantity);
    }

    match &req.unit {
        Some(None) => {
            qb.push(", unit = NULL");
        }
        Some(Some(val)) => {
            qb.push(", unit = ");
            qb.push_bind(val.clone());
        }
        None => {}
    }

    qb.push(" WHERE id = ");
    qb.push_bind(id);
    qb.push(" AND shopping_list_id = ");
    qb.push_bind(shopping_list_id);
    qb.push(" RETURNING id, shopping_list_id, item_id, name, quantity, unit, is_checked, created_at");

    qb.build_query_as::<TrackingShoppingListItem>()
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("Shopping list item not found".to_string()))
}

/// Remove a shopping list item by ID.
///
/// Both `id` and `shopping_list_id` must match for the delete to succeed,
/// preventing cross-list item manipulation.
pub async fn remove_list_item(
    pool: &PgPool,
    shopping_list_id: Uuid,
    id: Uuid,
) -> Result<(), AppError> {
    let result =
        sqlx::query("DELETE FROM tracking_shopping_list_items WHERE id = $1 AND shopping_list_id = $2")
            .bind(id)
            .bind(shopping_list_id)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Shopping list item not found".to_string()));
    }

    Ok(())
}

/// Toggle the is_checked state of a shopping list item.
///
/// Both `id` and `shopping_list_id` must match for the toggle to succeed,
/// preventing cross-list item manipulation.
pub async fn toggle_check(
    pool: &PgPool,
    shopping_list_id: Uuid,
    id: Uuid,
) -> Result<TrackingShoppingListItem, AppError> {
    sqlx::query_as::<_, TrackingShoppingListItem>(
        r#"UPDATE tracking_shopping_list_items
           SET is_checked = NOT is_checked
           WHERE id = $1 AND shopping_list_id = $2
           RETURNING id, shopping_list_id, item_id, name, quantity, unit, is_checked, created_at"#,
    )
    .bind(id)
    .bind(shopping_list_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound("Shopping list item not found".to_string()))
}
