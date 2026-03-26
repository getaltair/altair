use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::auth::middleware::AuthenticatedUser;
use crate::error::AppError;
use super::{
    models::{
        CreateShoppingListItemRequest, CreateShoppingListRequest, TrackingShoppingList,
        TrackingShoppingListItem, UpdateShoppingListItemRequest, UpdateShoppingListRequest,
    },
    service,
};

/// Query parameters for listing shopping lists
#[derive(Debug, Deserialize)]
pub struct ListShoppingListsQuery {
    pub household_id: Uuid,
}

/// Create a new shopping list
pub async fn create_list(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Json(body): Json<CreateShoppingListRequest>,
) -> Result<(StatusCode, Json<TrackingShoppingList>), AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Verify caller is a member of the household
    let household_ids = crate::auth::service::get_user_household_ids(&pool, auth.user_id).await?;
    if !household_ids.contains(&body.household_id) {
        return Err(AppError::Forbidden(
            "Not a member of this household".to_string(),
        ));
    }

    let list = service::create_list(&pool, auth.user_id, body).await?;
    Ok((StatusCode::CREATED, Json(list)))
}

/// List shopping lists for a household
pub async fn list_lists(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Query(query): Query<ListShoppingListsQuery>,
) -> Result<Json<Vec<TrackingShoppingList>>, AppError> {
    // Verify caller is a member of the household
    let household_ids = crate::auth::service::get_user_household_ids(&pool, auth.user_id).await?;
    if !household_ids.contains(&query.household_id) {
        return Err(AppError::Forbidden(
            "Not a member of this household".to_string(),
        ));
    }

    let lists = service::list_lists(&pool, query.household_id).await?;
    Ok(Json(lists))
}

/// Get a single shopping list by ID
pub async fn get_list(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<TrackingShoppingList>, AppError> {
    let list = service::get_list(&pool, id).await?;

    // Verify caller is a member of the household
    let household_ids = crate::auth::service::get_user_household_ids(&pool, auth.user_id).await?;
    if !household_ids.contains(&list.household_id) {
        return Err(AppError::Forbidden(
            "Not a member of this household".to_string(),
        ));
    }

    Ok(Json(list))
}

/// Update an existing shopping list
pub async fn update_list(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateShoppingListRequest>,
) -> Result<Json<TrackingShoppingList>, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Fetch the list to check household membership
    let existing = service::get_list(&pool, id).await?;
    let household_ids = crate::auth::service::get_user_household_ids(&pool, auth.user_id).await?;
    if !household_ids.contains(&existing.household_id) {
        return Err(AppError::Forbidden(
            "Not a member of this household".to_string(),
        ));
    }

    let list = service::update_list(&pool, id, body).await?;
    Ok(Json(list))
}

/// Delete a shopping list
pub async fn delete_list(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    // Fetch the list to check household membership
    let existing = service::get_list(&pool, id).await?;
    let household_ids = crate::auth::service::get_user_household_ids(&pool, auth.user_id).await?;
    if !household_ids.contains(&existing.household_id) {
        return Err(AppError::Forbidden(
            "Not a member of this household".to_string(),
        ));
    }

    service::delete_list(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Helper to get the parent shopping list and verify household membership
async fn verify_item_access(
    pool: &PgPool,
    shopping_list_id: Uuid,
    user_id: Uuid,
) -> Result<TrackingShoppingList, AppError> {
    let list = service::get_list(pool, shopping_list_id).await?;
    let household_ids = crate::auth::service::get_user_household_ids(pool, user_id).await?;
    if !household_ids.contains(&list.household_id) {
        return Err(AppError::Forbidden(
            "Not a member of this household".to_string(),
        ));
    }
    Ok(list)
}

/// Path parameters for item-level routes
#[derive(Debug, Deserialize)]
pub struct ListItemPath {
    pub id: Uuid,
    pub item_id: Uuid,
}

/// Add an item to a shopping list
pub async fn add_list_item(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(shopping_list_id): Path<Uuid>,
    Json(body): Json<CreateShoppingListItemRequest>,
) -> Result<(StatusCode, Json<TrackingShoppingListItem>), AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    verify_item_access(&pool, shopping_list_id, auth.user_id).await?;

    let item = service::add_list_item(&pool, shopping_list_id, body).await?;
    Ok((StatusCode::CREATED, Json(item)))
}

/// List all items in a shopping list
pub async fn list_list_items(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(shopping_list_id): Path<Uuid>,
) -> Result<Json<Vec<TrackingShoppingListItem>>, AppError> {
    verify_item_access(&pool, shopping_list_id, auth.user_id).await?;

    let items = service::list_list_items(&pool, shopping_list_id).await?;
    Ok(Json(items))
}

/// Update a shopping list item
pub async fn update_list_item(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(path): Path<ListItemPath>,
    Json(body): Json<UpdateShoppingListItemRequest>,
) -> Result<Json<TrackingShoppingListItem>, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    verify_item_access(&pool, path.id, auth.user_id).await?;

    let item = service::update_list_item(&pool, path.item_id, body).await?;
    Ok(Json(item))
}

/// Remove a shopping list item
pub async fn remove_list_item(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(path): Path<ListItemPath>,
) -> Result<StatusCode, AppError> {
    verify_item_access(&pool, path.id, auth.user_id).await?;

    service::remove_list_item(&pool, path.item_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Toggle the checked state of a shopping list item
pub async fn toggle_check(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(path): Path<ListItemPath>,
) -> Result<Json<TrackingShoppingListItem>, AppError> {
    verify_item_access(&pool, path.id, auth.user_id).await?;

    let item = service::toggle_check(&pool, path.item_id).await?;
    Ok(Json(item))
}
