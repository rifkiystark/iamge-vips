use std::sync::Arc;

use crate::handlers::process::process_image;
use axum::{extract, routing::get, Router};
use libvips::VipsApp;

pub fn create_routes(app: Arc<VipsApp>) -> Router {
    Router::new().route(
        "/process",
        get(process_image).layer(extract::Extension(app)),
    )
}
