use sqlx::PgPool;
use uuid::Uuid;

use super::models::{CreateCheckinRequest, DailyCheckin, UpdateCheckinRequest};
use crate::error::AppError;

pub async fn list_checkins(pool: &PgPool, user_id: Uuid) -> Result<Vec<DailyCheckin>, AppError> {
    let rows = sqlx::query_as::<_, DailyCheckin>(
        "SELECT id, user_id, checkin_date, energy_level, mood, notes, created_at, updated_at, deleted_at \
         FROM guidance_daily_checkins \
         WHERE user_id = $1 AND deleted_at IS NULL \
         ORDER BY checkin_date DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_checkin(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<DailyCheckin, AppError> {
    let row = sqlx::query_as::<_, DailyCheckin>(
        "SELECT id, user_id, checkin_date, energy_level, mood, notes, created_at, updated_at, deleted_at \
         FROM guidance_daily_checkins \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL",
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    row.ok_or(AppError::NotFound)
}

pub async fn create_checkin(
    pool: &PgPool,
    user_id: Uuid,
    req: CreateCheckinRequest,
) -> Result<DailyCheckin, AppError> {
    if !(1..=10).contains(&req.energy_level) {
        return Err(AppError::UnprocessableEntity(
            "energy_level must be between 1 and 10".to_string(),
        ));
    }

    let result = sqlx::query_as::<_, DailyCheckin>(
        "INSERT INTO guidance_daily_checkins (id, user_id, checkin_date, energy_level, mood, notes) \
         VALUES (gen_random_uuid(), $1, $2, $3, $4, $5) \
         RETURNING id, user_id, checkin_date, energy_level, mood, notes, created_at, updated_at, deleted_at",
    )
    .bind(user_id)
    .bind(req.checkin_date)
    .bind(req.energy_level)
    .bind(&req.mood)
    .bind(&req.notes)
    .fetch_one(pool)
    .await;

    match result {
        Ok(row) => Ok(row),
        Err(sqlx::Error::Database(ref db_err)) if db_err.code().as_deref() == Some("23505") => Err(
            AppError::Conflict("Check-in already exists for this date".to_string()),
        ),
        Err(e) => Err(e.into()),
    }
}

pub async fn update_checkin(
    pool: &PgPool,
    id: Uuid,
    user_id: Uuid,
    req: UpdateCheckinRequest,
) -> Result<DailyCheckin, AppError> {
    if req.energy_level.is_some_and(|l| !(1..=10).contains(&l)) {
        return Err(AppError::UnprocessableEntity(
            "energy_level must be between 1 and 10".to_string(),
        ));
    }

    let result = sqlx::query_as::<_, DailyCheckin>(
        "UPDATE guidance_daily_checkins \
         SET \
           checkin_date = COALESCE($3, checkin_date), \
           energy_level = COALESCE($4, energy_level), \
           mood         = COALESCE($5, mood), \
           notes        = COALESCE($6, notes), \
           updated_at   = NOW() \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL \
         RETURNING id, user_id, checkin_date, energy_level, mood, notes, created_at, updated_at, deleted_at",
    )
    .bind(id)
    .bind(user_id)
    .bind(req.checkin_date)
    .bind(req.energy_level)
    .bind(&req.mood)
    .bind(&req.notes)
    .fetch_optional(pool)
    .await;

    match result {
        Ok(Some(row)) => Ok(row),
        Ok(None) => Err(AppError::NotFound),
        Err(sqlx::Error::Database(ref db_err)) if db_err.code().as_deref() == Some("23505") => Err(
            AppError::Conflict("Check-in already exists for this date".to_string()),
        ),
        Err(e) => Err(e.into()),
    }
}

pub async fn delete_checkin(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query(
        "UPDATE guidance_daily_checkins \
         SET deleted_at = NOW(), updated_at = NOW() \
         WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL",
    )
    .bind(id)
    .bind(user_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests (S003-T) — sqlx::test integration tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::AppError;
    use crate::guidance::daily_checkins::models::{CreateCheckinRequest, UpdateCheckinRequest};
    use chrono::NaiveDate;
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

    // A-G-12: create check-in for a given date succeeds
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn create_checkin_succeeds(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "checkin_user@example.com").await;

        let result = create_checkin(
            &pool,
            user_id,
            CreateCheckinRequest {
                checkin_date: NaiveDate::from_ymd_opt(2026, 4, 15).unwrap(),
                energy_level: 7,
                mood: Some("good".to_string()),
                notes: Some("Feeling productive".to_string()),
            },
        )
        .await;

        let checkin = result.expect("create_checkin should succeed");
        assert_eq!(checkin.user_id, user_id);
        assert_eq!(
            checkin.checkin_date,
            NaiveDate::from_ymd_opt(2026, 4, 15).unwrap()
        );
        assert_eq!(checkin.energy_level, 7);
        assert_eq!(checkin.mood, Some("good".to_string()));
    }

    // A-G-12: second create for same user + same date returns Conflict
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn create_checkin_duplicate_date_returns_conflict(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "dup_checkin_user@example.com").await;

        let date = NaiveDate::from_ymd_opt(2026, 4, 15).unwrap();

        create_checkin(
            &pool,
            user_id,
            CreateCheckinRequest {
                checkin_date: date,
                energy_level: 5,
                mood: None,
                notes: None,
            },
        )
        .await
        .expect("First create should succeed");

        let result = create_checkin(
            &pool,
            user_id,
            CreateCheckinRequest {
                checkin_date: date,
                energy_level: 8,
                mood: Some("great".to_string()),
                notes: None,
            },
        )
        .await;

        assert!(
            matches!(result, Err(AppError::Conflict(_))),
            "Duplicate date for same user must return Conflict, got: {:?}",
            result
        );
    }

    // Create for same user different dates both succeed
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn create_checkins_different_dates_both_succeed(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "two_dates_user@example.com").await;

        create_checkin(
            &pool,
            user_id,
            CreateCheckinRequest {
                checkin_date: NaiveDate::from_ymd_opt(2026, 4, 14).unwrap(),
                energy_level: 6,
                mood: None,
                notes: None,
            },
        )
        .await
        .expect("First date should succeed");

        create_checkin(
            &pool,
            user_id,
            CreateCheckinRequest {
                checkin_date: NaiveDate::from_ymd_opt(2026, 4, 15).unwrap(),
                energy_level: 7,
                mood: None,
                notes: None,
            },
        )
        .await
        .expect("Second date should succeed");

        let list = list_checkins(&pool, user_id)
            .await
            .expect("list_checkins should succeed");
        assert_eq!(list.len(), 2, "Both checkins must be present");
    }

    // get_checkin returns NotFound for wrong user
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn get_checkin_wrong_user_returns_not_found(pool: PgPool) {
        let owner = Uuid::new_v4();
        let other = Uuid::new_v4();
        insert_test_user(&pool, owner, "owner_checkin@example.com").await;
        insert_test_user(&pool, other, "other_checkin@example.com").await;

        let checkin = create_checkin(
            &pool,
            owner,
            CreateCheckinRequest {
                checkin_date: NaiveDate::from_ymd_opt(2026, 4, 15).unwrap(),
                energy_level: 5,
                mood: None,
                notes: None,
            },
        )
        .await
        .expect("create should succeed");

        let result = get_checkin(&pool, checkin.id, other).await;

        assert!(
            matches!(result, Err(AppError::NotFound)),
            "get with wrong user_id must return NotFound, got: {:?}",
            result
        );
    }

    // update modifies energy_level correctly, leaves other fields unchanged
    #[sqlx::test(migrations = "../../../infra/migrations")]
    async fn update_energy_level_leaves_other_fields_unchanged(pool: PgPool) {
        let user_id = Uuid::new_v4();
        insert_test_user(&pool, user_id, "update_checkin@example.com").await;

        let checkin = create_checkin(
            &pool,
            user_id,
            CreateCheckinRequest {
                checkin_date: NaiveDate::from_ymd_opt(2026, 4, 15).unwrap(),
                energy_level: 5,
                mood: Some("okay".to_string()),
                notes: Some("Original notes".to_string()),
            },
        )
        .await
        .expect("create should succeed");

        let updated = update_checkin(
            &pool,
            checkin.id,
            user_id,
            UpdateCheckinRequest {
                checkin_date: None,
                energy_level: Some(9),
                mood: None,
                notes: None,
            },
        )
        .await
        .expect("update_checkin should succeed");

        assert_eq!(updated.energy_level, 9, "energy_level should be updated");
        assert_eq!(
            updated.mood,
            Some("okay".to_string()),
            "mood must remain unchanged"
        );
        assert_eq!(
            updated.notes,
            Some("Original notes".to_string()),
            "notes must remain unchanged"
        );
        assert_eq!(
            updated.checkin_date,
            NaiveDate::from_ymd_opt(2026, 4, 15).unwrap(),
            "checkin_date must remain unchanged"
        );
    }
}
