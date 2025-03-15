use dotenv::dotenv;
use std::env;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct EnvironmentApp {
    pub database_url: String,
    pub port: u16,
    pub is_prod: bool,
    pub jwt_secret: String,
    pub version: String,
}

impl EnvironmentApp {
    pub fn new() -> Self {
        dotenv().ok();
        info!("Environment variables loaded");

        let database_url = Self::get_env_var("DATABASE_URL");
        let jwt_secret = Self::get_env_var("JWT_SECRET");
        let version = Self::get_env_var("VERSION");

        let port = Self::get_env_var_with_default("APP_PORT", "3000")
            .parse::<u16>()
            .expect("APP_PORT must be a valid port number");

        let environment = Self::get_env_var("ENVIRONMENT");
        let is_prod = environment == "production";

        if is_prod {
            info!("Running in production mode");
        } else {
            warn!("Running in development mode");
        }

        Self {
            database_url,
            port,
            is_prod,
            jwt_secret,
            version,
        }
    }

    fn get_env_var(key: &str) -> String {
        env::var(key).unwrap_or_else(|_| panic!("{} must be set", key))
    }

    fn get_env_var_with_default(key: &str, default: &str) -> String {
        env::var(key).unwrap_or_else(|_| {
            warn!("{} not set, using default: {}", key, default);
            default.to_string()
        })
    }
}
