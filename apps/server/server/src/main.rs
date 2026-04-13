use axum::Router;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod config;
mod error;
mod routes;

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

    let app = Router::new().merge(routes::router());

    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    info!("Listening on {}", config.bind_addr);

    axum::serve(listener, app).await?;

    Ok(())
}
