use axum::Router;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Notify;
use tracing::info;

use crate::environment::EnvironmentApp;

pub async fn start_server(
    environment: EnvironmentApp,
    api_routes: Router,
    shutdown_signal: Arc<Notify>,
) -> Result<(), anyhow::Error> {
    let address = get_address_by_environment(environment.is_prod, environment.port);

    let listener = tokio::net::TcpListener::bind(&address).await?;
    info!("Server started on the address: {}", address);

    let shutdown_signal_clone = shutdown_signal.clone();
    let shutdown_future = async move {
        shutdown_signal_clone.notified().await;
        info!("Shutdown signal received, stopping server gracefully...");
    };

    axum::serve(listener, api_routes)
        .with_graceful_shutdown(shutdown_future)
        .await?;

    info!("Server stopped gracefully");
    Ok(())
}

fn get_address_by_environment(is_prod: bool, port: u16) -> SocketAddr {
    let ip = if is_prod {
        [0, 0, 0, 0] 
    } else {
        [127, 0, 0, 1]
    };

    SocketAddr::from((ip, port))
}
