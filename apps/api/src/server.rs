use axum::Router;
use std::net::SocketAddr;
use tracing::info;

use crate::environment::EnvironmentApp;

pub async fn start_server(
    environment: EnvironmentApp,
    api_routes: Router,
) -> Result<(), anyhow::Error> {
    let address = get_address_by_environment(environment.is_prod, environment.port);

    let listener = tokio::net::TcpListener::bind(&address).await?;

    info!("Server started on the address: {}", address);

    axum::serve(listener, api_routes).await?;

    Ok(())
}

fn get_address_by_environment(is_prod: bool, port: u16) -> SocketAddr {
    let ip = if is_prod {
        [0, 0, 0, 0] // 0.0.0.0
    } else {
        [127, 0, 0, 1] // 127.0.0.1
    };

    SocketAddr::from((ip, port))
}
