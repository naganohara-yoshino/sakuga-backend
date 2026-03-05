use mimalloc::MiMalloc;
use salvo::prelude::*;
use server::{api::router::app_router, config::AppConfig, infrastructure};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    // Initialize logging subsystem
    tracing_subscriber::fmt().init();

    let config = AppConfig::load().expect("Failed to load config");

    let app_state = infrastructure::init(&config)
        .await
        .expect("Failed to initialize app state");

    let router = app_router(app_state);

    let acceptor = TcpListener::new(config.server_address()).bind().await;
    // Start serving requests
    Server::new(acceptor).serve(router).await;
}
