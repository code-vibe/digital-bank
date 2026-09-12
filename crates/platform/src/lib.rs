use dotenvy::dotenv;
use std::env;

pub fn health() -> &'static str {
    "platform ok"
}

pub struct Config {
    pub port: u16,
    pub host: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let port: u16 = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .expect("PORT must be a valid u16");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Self {
            host,
            port,
            database_url,
        }
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
