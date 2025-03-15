use sqlx::{Pool, Postgres, migrate::Migrator, postgres::PgPoolOptions};
use std::path::Path;
use tracing::{error, info};

use crate::environment::EnvironmentApp;

pub type AppDBPool = Pool<Postgres>;

#[derive(Debug, Clone)]
pub struct DatabaseApp {
    pub pool: AppDBPool,
}

impl DatabaseApp {
    pub async fn new(environment: &EnvironmentApp) -> Result<Self, sqlx::Error> {
        let pool = Self::create_pool(&environment.database_url).await?;
        info!("Database connection pool created");
        Ok(Self { pool })
    }

    pub async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        const MIGRATIONS_DIR: &str = "migrations";

        let directory = Path::new(MIGRATIONS_DIR);
        if !directory.exists() {
            error!("Migrations directory '{}' not found", MIGRATIONS_DIR);
            return Err(sqlx::Error::Configuration(
                format!("Migrations directory '{}' does not exist", MIGRATIONS_DIR).into(),
            ));
        }

        info!("Running database migrations from '{}'", MIGRATIONS_DIR);
        Migrator::new(directory).await?.run(&self.pool).await?;
        info!("Database migrations completed successfully");

        Ok(())
    }

    pub async fn create_pool(url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
        PgPoolOptions::new().max_connections(20).connect(url).await
    }

    pub async fn close_pool(&self) -> Result<(), sqlx::Error> {
        info!("Closing database connection pool");
        Ok(self.pool.close().await)
    }
}

impl Drop for DatabaseApp {
    fn drop(&mut self) {
        let pool = self.pool.clone();
        tokio::spawn(async move {
            pool.close().await;
            info!("Database connection pool closed during drop");
        });
    }
}
