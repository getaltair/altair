use sqlx::PgPool;
use uuid::Uuid;

use super::models::{
    CreateLocationRequest, TrackingLocation, TrackingLocationRow, UpdateLocationRequest,
};
use crate::error::AppError;
use crate::tracking::household::assert_household_member;

pub async fn list_locations(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
) -> Result<Vec<TrackingLocation>, AppError> {
    assert_household_member(pool, user_id, household_id).await?;

    let rows = sqlx::query_as::<_, TrackingLocationRow>(
        "SELECT id, household_id, name, created_at, updated_at, deleted_at \
         FROM tracking_locations \
         WHERE household_id = $1 AND deleted_at IS NULL \
         ORDER BY name ASC",
    )
    .bind(household_id)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(TrackingLocation::from).collect())
}

pub async fn get_location(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
    location_id: Uuid,
) -> Result<TrackingLocation, AppError> {
    assert_household_member(pool, user_id, household_id).await?;

    let row = sqlx::query_as::<_, TrackingLocationRow>(
        "SELECT id, household_id, name, created_at, updated_at, deleted_at \
         FROM tracking_locations \
         WHERE id = $1 AND household_id = $2 AND deleted_at IS NULL",
    )
    .bind(location_id)
    .bind(household_id)
    .fetch_optional(pool)
    .await?;

    row.map(TrackingLocation::from).ok_or(AppError::NotFound)
}

pub async fn create_location(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateLocationRequest,
) -> Result<TrackingLocation, AppError> {
    assert_household_member(pool, user_id, req.household_id).await?;

    let row = sqlx::query_as::<_, TrackingLocationRow>(
        "INSERT INTO tracking_locations (household_id, name) \
         VALUES ($1, $2) \
         RETURNING id, household_id, name, created_at, updated_at, deleted_at",
    )
    .bind(req.household_id)
    .bind(&req.name)
    .fetch_one(pool)
    .await?;

    Ok(TrackingLocation::from(row))
}

pub async fn update_location(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
    location_id: Uuid,
    req: UpdateLocationRequest,
) -> Result<TrackingLocation, AppError> {
    assert_household_member(pool, user_id, household_id).await?;

    let row = sqlx::query_as::<_, TrackingLocationRow>(
        "UPDATE tracking_locations \
         SET name = COALESCE($3, name), updated_at = NOW() \
         WHERE id = $1 AND household_id = $2 AND deleted_at IS NULL \
         RETURNING id, household_id, name, created_at, updated_at, deleted_at",
    )
    .bind(location_id)
    .bind(household_id)
    .bind(&req.name)
    .fetch_optional(pool)
    .await?;

    row.map(TrackingLocation::from).ok_or(AppError::NotFound)
}

pub async fn delete_location(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
    location_id: Uuid,
) -> Result<(), AppError> {
    assert_household_member(pool, user_id, household_id).await?;

    let result = sqlx::query(
        "UPDATE tracking_locations \
         SET deleted_at = NOW(), updated_at = NOW() \
         WHERE id = $1 AND household_id = $2 AND deleted_at IS NULL",
    )
    .bind(location_id)
    .bind(household_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests (S003-T)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::AppError;
    use chrono::{DateTime, Utc};
    use sqlx::PgPool;

    async fn insert_test_user(pool: &PgPool, user_id: Uuid, email: &str) {
        sqlx::query(
            "INSERT INTO users (id, email, display_name, password_hash, is_admin, status) \
             VALUES ($1, $2, 'Test User', 'hashed_password', false, 'active')",
        )
        .bind(user_id)
        .bind(email)
        .execute(pool)
        .await
        .expect("Failed to insert test user");
    }

    async fn insert_test_household(pool: &PgPool, household_id: Uuid, owner_id: Uuid) {
        sqlx::query(
            "INSERT INTO households (id, owner_id, name) VALUES ($1, $2, 'Test Household')",
        )
        .bind(household_id)
        .bind(owner_id)
        .execute(pool)
        .await
        .expect("Failed to insert test household");
    }

    async fn insert_membership(pool: &PgPool, household_id: Uuid, user_id: Uuid) {
        sqlx::query("INSERT INTO household_memberships (household_id, user_id) VALUES ($1, $2)")
            .bind(household_id)
            .bind(user_id)
            .execute(pool)
            .await
            .expect("Failed to insert household membership");
    }

    /// FA-001: A non-member gets Forbidden for list.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn non_member_list_returns_forbidden(pool: PgPool) {
        let owner = Uuid::new_v4();
        let non_member = Uuid::new_v4();
        let household_id = Uuid::new_v4();

        insert_test_user(&pool, owner, "owner@example.com").await;
        insert_test_user(&pool, non_member, "nonmember@example.com").await;
        insert_test_household(&pool, household_id, owner).await;
        insert_membership(&pool, household_id, owner).await;

        let result = list_locations(&pool, non_member, household_id).await;
        assert!(
            matches!(result, Err(AppError::Forbidden)),
            "non-member list must return Forbidden, got: {:?}",
            result
        );
    }

    /// FA-001: A non-member gets Forbidden for create.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn non_member_create_returns_forbidden(pool: PgPool) {
        let owner = Uuid::new_v4();
        let non_member = Uuid::new_v4();
        let household_id = Uuid::new_v4();

        insert_test_user(&pool, owner, "owner2@example.com").await;
        insert_test_user(&pool, non_member, "nonmember2@example.com").await;
        insert_test_household(&pool, household_id, owner).await;
        insert_membership(&pool, household_id, owner).await;

        let result = create_location(
            &pool,
            non_member,
            CreateLocationRequest {
                name: "Garage".to_string(),
                household_id,
            },
        )
        .await;
        assert!(
            matches!(result, Err(AppError::Forbidden)),
            "non-member create must return Forbidden, got: {:?}",
            result
        );
    }

    /// FA-002: Creating a location as a member returns the created location with matching fields.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn member_create_returns_location(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "member@example.com").await;
        insert_test_household(&pool, household_id, user_id).await;
        insert_membership(&pool, household_id, user_id).await;

        let location = create_location(
            &pool,
            user_id,
            CreateLocationRequest {
                name: "Kitchen".to_string(),
                household_id,
            },
        )
        .await
        .expect("create_location failed");

        assert_eq!(location.name, "Kitchen");
        assert!(!location.id.is_nil());
    }

    /// FA-003: Listing locations returns only locations belonging to the queried household.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn list_returns_only_own_household_locations(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_a = Uuid::new_v4();
        let household_b = Uuid::new_v4();

        insert_test_user(&pool, user_id, "isolate@example.com").await;
        insert_test_household(&pool, household_a, user_id).await;
        insert_test_household(&pool, household_b, user_id).await;
        insert_membership(&pool, household_a, user_id).await;
        insert_membership(&pool, household_b, user_id).await;

        create_location(
            &pool,
            user_id,
            CreateLocationRequest {
                name: "Pantry".to_string(),
                household_id: household_a,
            },
        )
        .await
        .expect("create for household_a failed");

        create_location(
            &pool,
            user_id,
            CreateLocationRequest {
                name: "Basement".to_string(),
                household_id: household_b,
            },
        )
        .await
        .expect("create for household_b failed");

        let a_list = list_locations(&pool, user_id, household_a)
            .await
            .expect("list_locations for household_a failed");
        let b_list = list_locations(&pool, user_id, household_b)
            .await
            .expect("list_locations for household_b failed");

        assert_eq!(a_list.len(), 1, "household_a must have exactly 1 location");
        assert_eq!(a_list[0].name, "Pantry");

        assert_eq!(b_list.len(), 1, "household_b must have exactly 1 location");
        assert_eq!(b_list[0].name, "Basement");
    }

    /// FA-017: Soft-deleted location is absent from list results.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn soft_deleted_location_absent_from_list(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "softdelete@example.com").await;
        insert_test_household(&pool, household_id, user_id).await;
        insert_membership(&pool, household_id, user_id).await;

        let location = create_location(
            &pool,
            user_id,
            CreateLocationRequest {
                name: "Attic".to_string(),
                household_id,
            },
        )
        .await
        .expect("create_location failed");

        delete_location(&pool, user_id, household_id, location.id)
            .await
            .expect("delete_location failed");

        let list = list_locations(&pool, user_id, household_id)
            .await
            .expect("list_locations failed");

        assert!(
            list.is_empty(),
            "soft-deleted location must not appear in list results"
        );

        // Verify the row still physically exists in the database with deleted_at set.
        let deleted_at: Option<DateTime<Utc>> =
            sqlx::query_scalar("SELECT deleted_at FROM tracking_locations WHERE id = $1")
                .bind(location.id)
                .fetch_one(&pool)
                .await
                .expect("Row must still exist after soft delete");
        assert!(
            deleted_at.is_some(),
            "deleted_at must be non-null after soft delete"
        );
    }
}
