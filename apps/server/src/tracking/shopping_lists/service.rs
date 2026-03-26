use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use super::models::{
    CreateShoppingListItemRequest, CreateShoppingListRequest, TrackingShoppingList,
    TrackingShoppingListItem, UpdateShoppingListItemRequest, UpdateShoppingListRequest,
};

/// Create a new shopping list for the given user and household
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
    .map_err(AppError::Database)
}

/// List all shopping lists for a household
pub async fn list_lists(
    pool: &PgPool,
    household_id: Uuid,
) -> Result<Vec<TrackingShoppingList>, AppError> {
    sqlx::query_as::<_, TrackingShoppingList>(
        r#"SELECT id, user_id, household_id, name, status, created_at, updated_at
           FROM tracking_shopping_lists
           WHERE household_id = $1
           ORDER BY created_at DESC"#,
    )
    .bind(household_id)
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

/// Update an existing shopping list (partial update)
pub async fn update_list(
    pool: &PgPool,
    id: Uuid,
    req: UpdateShoppingListRequest,
) -> Result<TrackingShoppingList, AppError> {
    let mut qb: sqlx::QueryBuilder<sqlx::Postgres> =
        sqlx::QueryBuilder::new("UPDATE tracking_shopping_lists SET updated_at = now()");

    if let Some(ref name) = req.name {
        qb.push(", name = ");
        qb.push_bind(name.clone());
    }

    if let Some(ref status) = req.status {
        qb.push(", status = ");
        qb.push_bind(status.clone());
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

/// Delete a shopping list by ID
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

/// Add an item to a shopping list
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
    .map_err(AppError::Database)
}

/// List all items in a shopping list
pub async fn list_list_items(
    pool: &PgPool,
    shopping_list_id: Uuid,
) -> Result<Vec<TrackingShoppingListItem>, AppError> {
    sqlx::query_as::<_, TrackingShoppingListItem>(
        r#"SELECT id, shopping_list_id, item_id, name, quantity, unit, is_checked, created_at
           FROM tracking_shopping_list_items
           WHERE shopping_list_id = $1
           ORDER BY created_at ASC"#,
    )
    .bind(shopping_list_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}

/// Update a shopping list item (partial update)
pub async fn update_list_item(
    pool: &PgPool,
    id: Uuid,
    req: UpdateShoppingListItemRequest,
) -> Result<TrackingShoppingListItem, AppError> {
    let mut qb: sqlx::QueryBuilder<sqlx::Postgres> =
        sqlx::QueryBuilder::new("UPDATE tracking_shopping_list_items SET id = id");

    if let Some(ref name) = req.name {
        qb.push(", name = ");
        qb.push_bind(name.clone());
    }

    if let Some(quantity) = req.quantity {
        qb.push(", quantity = ");
        qb.push_bind(quantity);
    }

    if let Some(ref unit) = req.unit {
        qb.push(", unit = ");
        qb.push_bind(unit.clone());
    }

    qb.push(" WHERE id = ");
    qb.push_bind(id);
    qb.push(" RETURNING id, shopping_list_id, item_id, name, quantity, unit, is_checked, created_at");

    qb.build_query_as::<TrackingShoppingListItem>()
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("Shopping list item not found".to_string()))
}

/// Remove a shopping list item by ID
pub async fn remove_list_item(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM tracking_shopping_list_items WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Shopping list item not found".to_string()));
    }

    Ok(())
}

/// Toggle the is_checked state of a shopping list item
pub async fn toggle_check(
    pool: &PgPool,
    id: Uuid,
) -> Result<TrackingShoppingListItem, AppError> {
    sqlx::query_as::<_, TrackingShoppingListItem>(
        r#"UPDATE tracking_shopping_list_items
           SET is_checked = NOT is_checked
           WHERE id = $1
           RETURNING id, shopping_list_id, item_id, name, quantity, unit, is_checked, created_at"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound("Shopping list item not found".to_string()))
}
