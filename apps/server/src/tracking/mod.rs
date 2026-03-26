pub mod categories;
pub mod items;
pub mod locations;
pub mod shopping_lists;

use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::service::get_user_household_ids;
use crate::error::AppError;

/// Query parameters for paginating list endpoints.
///
/// Both fields are optional: clients that omit them receive sensible
/// defaults (limit 100, offset 0). The maximum allowed limit is 500.
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl PaginationParams {
    pub fn limit_or_default(&self) -> i64 {
        self.limit.unwrap_or(100).min(500)
    }
    pub fn offset_or_default(&self) -> i64 {
        self.offset.unwrap_or(0).max(0)
    }
}

/// Verify the authenticated user is a member of the given household.
///
/// Shared helper extracted from per-module handler code so that all
/// tracking sub-modules can reuse the same membership check.
pub async fn verify_household_membership(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
) -> Result<(), AppError> {
    let household_ids = get_user_household_ids(pool, user_id).await?;
    if !household_ids.contains(&household_id) {
        return Err(AppError::Forbidden(
            "Not a member of this household".to_string(),
        ));
    }
    Ok(())
}
