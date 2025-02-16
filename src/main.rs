use libvips::VipsApp;

mod handlers;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    let app = VipsApp::new("VipsApp", true).expect("Failed to initialize VipsApp");
    app.concurrency_set(2);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes::create_routes())
        .await
        .unwrap();
}
