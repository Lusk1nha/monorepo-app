use sqlx::{Error, Pool, Postgres, migrate::Migrator, postgres::PgPoolOptions};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};
use tracing::{error, info};

use crate::environment::EnvironmentApp;

pub type AppDBPool = Pool<Postgres>;

#[derive(Debug, Clone)]
pub struct DatabaseApp {
    pub pool: Arc<Mutex<AppDBPool>>,
    environment: EnvironmentApp,
}

impl DatabaseApp {
    pub async fn new(environment: &EnvironmentApp) -> Result<Self, Error> {
        let pool = Self::create_pool(&environment.database_url).await?;
        let db_app = Self {
            pool: Arc::new(Mutex::new(pool)),
            environment: environment.clone(),
        };

        db_app.start_auto_reconnect();

        info!("Database connection pool created");
        Ok(db_app)
    }

    pub async fn run_migrations(&self) -> Result<(), Error> {
        const MIGRATIONS_DIR: &str = "migrations";

        let source_dir = self.environment.manifest_dir.clone();
        let join_current = PathBuf::from(source_dir).join(MIGRATIONS_DIR);

        let directory = Path::new(&join_current);
        if !directory.exists() {
            error!("Migrations directory '{}' not found", MIGRATIONS_DIR);

            return Err(Error::Configuration(
                format!("Migrations directory '{}' does not exist", MIGRATIONS_DIR).into(),
            ));
        }

        info!("Running database migrations from '{}'", MIGRATIONS_DIR);
        let pool = self.pool.lock().await; // Bloqueia o pool para uso seguro.
        Migrator::new(directory).await?.run(&*pool).await?;
        info!("Database migrations completed successfully");

        Ok(())
    }

    pub async fn create_pool(url: &str) -> Result<Pool<Postgres>, Error> {
        PgPoolOptions::new().max_connections(20).connect(url).await
    }

    pub async fn close_pool(&self) -> Result<(), Error> {
        info!("Closing database connection pool");
        let pool = self.pool.lock().await;
        pool.close().await;
        Ok(())
    }

    fn start_auto_reconnect(&self) {
        let pool = Arc::clone(&self.pool);
        let url = self.environment.database_url.clone();

        tokio::spawn(async move {
            loop {
                // Verifica se o pool está fechado.
                let is_closed = {
                    let pool = pool.lock().await;
                    pool.is_closed()
                };

                if is_closed {
                    info!("Attempting to reconnect to the database...");

                    match Self::create_pool(&url).await {
                        Ok(new_pool) => {
                            info!("Reconnected to the database.");
                            // Substitui o pool antigo pelo novo.
                            let mut pool = pool.lock().await;
                            *pool = new_pool;
                        }
                        Err(e) => {
                            error!("Failed to reconnect: {:?}", e);
                        }
                    }
                }

                // Espera um tempo antes de verificar novamente.
                sleep(Duration::from_secs(10)).await;
            }
        });
    }
}

impl Drop for DatabaseApp {
    fn drop(&mut self) {
        let pool = Arc::clone(&self.pool);
        tokio::spawn(async move {
            let pool = pool.lock().await;
            pool.close().await;
            info!("Database connection pool closed during drop");
        });
    }
}
