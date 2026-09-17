use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use platform::Config;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

use axum::Json;
use identity::RegisterRequest;

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
        .route("/register", post(register))
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

//#[axum::debug_handler]
async fn register(
    State(pool): State<PgPool>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<identity::RegisterResponse>, StatusCode> {
    let input = RegisterRequest {
        email: request.email,
        password: request.password,
    };

    let user = identity::register(&pool, input)
        .await
        .map_err(|error| match error {
            identity::IdentityError::EmailAlreadyExists => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(user))
}
