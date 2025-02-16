use crate::handlers::process::process_image;
use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    Router::new().route("/process", get(process_image))
}
