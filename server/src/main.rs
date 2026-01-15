use salvo::prelude::*;
use server::api::router::app_router;

#[tokio::main]
async fn main() {
    // Initialize logging subsystem
    tracing_subscriber::fmt().init();

    // Bind server to port 8698
    let acceptor = TcpListener::new("0.0.0.0:8698").bind().await;

    let router = app_router();

    // Start serving requests
    Server::new(acceptor).serve(router).await;
}
