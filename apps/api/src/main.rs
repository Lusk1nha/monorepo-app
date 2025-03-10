use api::{
    api_state::AppState, database::DatabaseApp, environment::EnvironmentApp,
    logging::tracing::init_logger, router::create_api_routes, server::start_server,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = init_logger();

    let environment = EnvironmentApp::new();

    let database = DatabaseApp::new(&environment).await?;
    database.run_migrations().await?;

    let app_state = AppState::new(database.clone(), environment.clone())?;
    let api_routes = create_api_routes(app_state);

    let server_task = tokio::spawn(start_server(environment, api_routes));

    tokio::select! {
        res = server_task => {
            if let Err(e) = res {
                tracing::error!("Server task failed: {}", e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            tracing::warn!("Received shutdown signal");
        }
    }

    // Close the database connection pool
    database.close_pool().await?;

    Ok(())
}
