use axum::Router;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod config;
mod error;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = config::Config::from_env()?;

    let app = Router::new().merge(routes::router());

    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    info!("Listening on {}", config.bind_addr);

    axum::serve(listener, app).await?;

    Ok(())
}
