use sqlx::postgres::{PgPool, PgPoolOptions};

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

pub async fn connect(url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new().max_connections(8).connect(url).await?;
    MIGRATOR.run(&pool).await?;
    Ok(pool)
}
