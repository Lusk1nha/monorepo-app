use log_service::LogService;


#[tokio::main]	
async fn main() {
    let _guard = LogService::default().unwrap();

    println!("Hello, world!");
    tracing::info!("Hello, world!");
}
