use altair_server::{auth, build_app_state, config, core, db, guidance, knowledge, routes, sync, tracking};
use anyhow::Context;
use axum::Router;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dotenv_result = dotenvy::dotenv();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    match dotenv_result {
        Ok(_) => {}
        Err(e) if e.not_found() => {
            tracing::debug!(".env file not found — reading environment variables directly");
        }
        Err(e) => {
            tracing::warn!("Failed to parse .env file: {}", e);
        }
    }

    let config = config::Config::from_env()?;

    // --- Build database pool and run migrations ---
    let pool = db::build_pool(&config.database_url)
        .await
        .context("Failed to connect to database")?;
    db::run_migrations(&pool)
        .await
        .context("Failed to run database migrations")?;

    // --- Parse RSA private key and build jwt keys + JWKS ---
    let app_state = build_app_state(pool, &config.jwt_private_key_pem, config.secure_cookies)
        .context("Failed to initialise application state from JWT_PRIVATE_KEY")?;

    let app: Router = Router::new()
        .merge(auth::auth_router())
        .merge(core::initiatives::router())
        .merge(core::tags::router())
        .merge(core::relations::router())
        .merge(knowledge::router())
        .merge(sync::router())
        .merge(guidance::router())
        .merge(tracking::router())
        .merge(routes::router().with_state(()))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    info!("Listening on {}", config.bind_addr);

    axum::serve(listener, app).await?;

    Ok(())
}
