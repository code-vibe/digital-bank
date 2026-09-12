use axum::Router;
use axum::routing::get;
use tracing::info;
use platform::Config;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();
    info!("tracing initialized");
    

    let config = Config::from_env();
    let address = Config::bind_address(&config);


    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz));

    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("failed to bind TCP listener");

    info!("API listening on {}", address);

    axum::serve(listener, app)
        .await
        .expect("server failed");


}

async fn healthz() -> &'static str {
    platform::health()
}

async fn readyz() -> &'static str {
    platform::health()
}