use std::env;

use dotenv::dotenv;

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

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let port = env::var("APP_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .expect("APP_PORT must be a number");

        let environment = env::var("ENVIRONMENT").expect("ENVIRONMENT must be set");

        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let version = env::var("VERSION").expect("VERSION must be set");

        Self {
            database_url,
            port,

            is_prod: environment == "production",
            jwt_secret,
            version,
        }
    }
}
