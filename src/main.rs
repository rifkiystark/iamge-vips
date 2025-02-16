use libvips::VipsApp;
use std::{env, sync::Arc};

mod handlers;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    let app = Arc::new(VipsApp::new("VipsApp", true).expect("Failed to initialize VipsApp"));
    app.concurrency_set(2);

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes::create_routes(app))
        .await
        .unwrap();
}
