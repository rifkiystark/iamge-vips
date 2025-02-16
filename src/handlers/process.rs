use axum::{
    extract::Query,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Extension,
};
use libvips::VipsApp;
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};

use crate::services::image;

#[derive(Deserialize)]
pub struct ProcessParams {
    url: String,
    width: Option<u32>,
    height: Option<u32>,
    format: Option<String>,
    rotate: Option<f64>,
    grayscale: Option<bool>,
}

pub async fn process_image(
    Extension(app): Extension<Arc<VipsApp>>,
    Query(params): Query<ProcessParams>,
) -> impl IntoResponse {
    let mut options: HashMap<&str, String> = HashMap::new();
    options.insert("url", params.url.clone());

    for (key, value) in [
        ("width", params.width.map(|v| v.to_string())),
        ("height", params.height.map(|v| v.to_string())),
        ("format", params.format.clone()),
        ("rotate", params.rotate.map(|v| v.to_string())),
        ("grayscale", params.grayscale.map(|v| v.to_string())),
    ]
    .iter()
    .filter_map(|(k, v)| v.as_ref().map(|v| (*k, v.clone())))
    {
        options.insert(key, value);
    }

    match image::process(&options).await {
        Ok(result) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                "content-type",
                HeaderValue::from_str(&result.mime_type).unwrap(),
            );
            (StatusCode::OK, headers, result.image).into_response()
        }
        Err(err) => {
            let app_err = app.error_buffer().unwrap();
            format!("Error processing image: {} {}", app_err, err).into_response()
        }
    }
}
