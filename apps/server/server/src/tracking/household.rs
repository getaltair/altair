use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;

/// Verify that `user_id` is an active member of `household_id`.
///
/// Returns `Ok(())` if a matching active membership row exists.
/// Returns `AppError::Forbidden` if the user is not a member, the membership
/// has been soft-deleted, or the household does not exist.
pub async fn assert_household_member(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
) -> Result<(), AppError> {
    let found = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(\
            SELECT 1 FROM household_memberships \
            WHERE household_id = $1 AND user_id = $2 AND deleted_at IS NULL\
         )",
    )
    .bind(household_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    if found {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

// ---------------------------------------------------------------------------
// Tests (S002-T)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
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

    /// An active member of a household gets Ok(()).
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn member_returns_ok(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "member@example.com").await;
        insert_test_household(&pool, household_id, user_id).await;
        insert_membership(&pool, household_id, user_id).await;

        let result = assert_household_member(&pool, user_id, household_id).await;
        assert!(result.is_ok(), "active member must return Ok(())");
    }

    /// A user with no membership row gets Forbidden.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn non_member_returns_forbidden(pool: PgPool) {
        let owner_id = Uuid::new_v4();
        let non_member_id = Uuid::new_v4();
        let household_id = Uuid::new_v4();

        insert_test_user(&pool, owner_id, "owner@example.com").await;
        insert_test_user(&pool, non_member_id, "nonmember@example.com").await;
        insert_test_household(&pool, household_id, owner_id).await;
        insert_membership(&pool, household_id, owner_id).await;
        // non_member_id has no membership row

        let result = assert_household_member(&pool, non_member_id, household_id).await;
        assert!(
            matches!(result, Err(AppError::Forbidden)),
            "non-member must return Forbidden, got: {:?}",
            result
        );
    }

    /// A non-existent household_id gets Forbidden (not 500).
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn unknown_household_returns_forbidden(pool: PgPool) {
        let user_id = Uuid::new_v4();
        let unknown_household_id = Uuid::new_v4();

        insert_test_user(&pool, user_id, "user@example.com").await;
        // No household row inserted — household_id is unknown

        let result = assert_household_member(&pool, user_id, unknown_household_id).await;
        assert!(
            matches!(result, Err(AppError::Forbidden)),
            "unknown household must return Forbidden, got: {:?}",
            result
        );
    }

    /// A membership belonging to user A does not grant access for user B.
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn wrong_user_id_returns_forbidden(pool: PgPool) {
        let user_a = Uuid::new_v4();
        let user_b = Uuid::new_v4();
        let household_id = Uuid::new_v4();

        insert_test_user(&pool, user_a, "user_a@example.com").await;
        insert_test_user(&pool, user_b, "user_b@example.com").await;
        insert_test_household(&pool, household_id, user_a).await;
        insert_membership(&pool, household_id, user_a).await;
        // user_b has no membership in this household

        let result = assert_household_member(&pool, user_b, household_id).await;
        assert!(
            matches!(result, Err(AppError::Forbidden)),
            "user without membership must return Forbidden, got: {:?}",
            result
        );
    }
}
