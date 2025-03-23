use dotenv::dotenv;
use std::env;
use tracing::{info, warn};

use mail_service::SMTPConfig;

#[derive(Debug, Clone)]
pub struct EnvironmentApp {
    pub manifest_dir: String,

    pub database_url: String,
    pub port: u16,
    pub is_prod: bool,
    pub jwt_secret: String,
    pub version: String,

    pub smtp_config: SMTPConfig,
}

impl EnvironmentApp {
    pub fn new() -> Self {
        dotenv().ok();
        info!("Environment variables loaded");

        let manifest_dir = Self::get_env_var("CARGO_MANIFEST_DIR");

        let database_url = Self::get_env_var("DATABASE_URL");
        let jwt_secret = Self::get_env_var("JWT_SECRET");
        let version = Self::get_env_var("VERSION");

        let port = Self::get_env_var_with_default("APP_PORT", "3000")
            .parse::<u16>()
            .expect("APP_PORT must be a valid port number");

        let environment = Self::get_env_var("ENVIRONMENT");
        let is_prod = environment == "production";

        let smtp_config = Self::get_smpt_config();

        if is_prod {
            info!("Running in production mode");
        } else {
            warn!("Running in development mode");
        }

        Self {
            manifest_dir,

            database_url,
            port,
            is_prod,
            jwt_secret,
            version,

            smtp_config,
        }
    }

    fn get_smpt_config() -> SMTPConfig {
        let smtp_server = Self::get_env_var("SMTP_SERVER");
        let smtp_port = Self::get_env_var_with_default("SMTP_PORT", "587")
            .parse::<u16>()
            .expect("SMTP_PORT must be a valid port number");

        let smtp_username = Self::get_env_var("SMTP_USERNAME");
        let smtp_password = Self::get_env_var("SMTP_PASSWORD");

        SMTPConfig {
            smtp_server,
            smtp_port,
            smtp_username,
            smtp_password,
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
