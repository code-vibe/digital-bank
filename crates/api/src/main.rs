use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use platform::Config;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();
    info!("tracing initialized");

    let config = Config::from_env();
    let address = config.bind_address();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&config.database_url)
        .unwrap();

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .with_state(pool.clone());

    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("failed to bind TCP listener");

    info!("API listening on {}", address);

    axum::serve(listener, app).await.expect("server failed");
}

async fn healthz() -> &'static str {
    platform::health()
}

async fn readyz(State(pool): State<PgPool>) -> Result<&'static str, StatusCode> {
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    Ok("OK")
}
