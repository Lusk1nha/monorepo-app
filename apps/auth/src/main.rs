use auth::{
    api_state::AppState, database::DatabaseApp, environment::EnvironmentApp,
    logging::tracing::init_logger, router::create_api_routes, server::start_server,
};
use std::sync::Arc;
use tokio::sync::Notify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = init_logger();
    tracing::info!("Starting application initialization");

    let environment = EnvironmentApp::new();
    tracing::debug!("Environment loaded");

    let database = DatabaseApp::new(&environment).await?;
    tracing::info!("Database connection pool created");

    database.run_migrations().await?;
    tracing::info!("Database migrations executed");

    let app_state = AppState::new(
        database.clone(), // Clone seguro se DatabaseApp usar Arc internamente
        environment.clone(),
    )?;
    tracing::debug!("Application state initialized");

    let api_routes = create_api_routes(app_state);
    tracing::debug!("API routes configured");

    let shutdown_signal = Arc::new(Notify::new());

    let server_task = tokio::spawn({
        let shutdown_signal = shutdown_signal.clone();
        async move { start_server(environment, api_routes, shutdown_signal).await }
    });

    tokio::select! {
        res = server_task => {
            match res {
                Ok(Ok(_)) => tracing::info!("Server shutdown normally"),
                Ok(Err(e)) => {
                    tracing::error!(error = %e, "Server task failed");
                    return Err(e.into());
                }
                Err(join_err) => {
                    tracing::error!(error = %join_err, "Server task panicked");
                    return Err(join_err.into());
                }
            }
        }
        _ = tokio::signal::ctrl_c() => {
            tracing::warn!("Received SIGINT signal, initiating graceful shutdown");
            shutdown_signal.notify_one();
        }
    }

    database.close_pool().await?;
    tracing::info!("Database connection pool closed");

    Ok(())
}
