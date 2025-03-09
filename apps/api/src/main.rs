use api::{database::DatabaseApp, environment::EnvironmentApp, logging::tracing::init_logger};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = init_logger();

    let environment = EnvironmentApp::new();

    let database = DatabaseApp::new(&environment).await?;
    database.run_migrations().await?;

    Ok(())
}
