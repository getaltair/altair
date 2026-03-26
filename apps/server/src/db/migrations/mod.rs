use sqlx::PgPool;

use crate::error::AppError;

/// Run database migrations from the `migrations/` directory
///
/// Uses sqlx's built-in migration runner to apply all pending SQL migrations
/// in order. The `_sqlx_migrations` table tracks which migrations have been
/// applied.
///
/// # Arguments
///
/// * `pool` - PostgreSQL connection pool
///
/// # Returns
///
/// * `Ok(())` on successful migration or if already up-to-date
/// * `Err(AppError)` if migration fails
pub async fn run_migrations(pool: &PgPool) -> Result<(), AppError> {
    tracing::info!("Running database migrations...");

    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| AppError::Internal(format!("Migration failed: {}", e)))?;

    tracing::info!("Database migrations completed successfully");
    Ok(())
}
