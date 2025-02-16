use libvips::{
    ops::{
        self, JpegsaveBufferOptions, PngsaveBufferOptions, ThumbnailImageOptions,
        WebpsaveBufferOptions,
    },
    VipsImage,
};
use reqwest;
use std::collections::HashMap;
use tokio::time::Instant;

pub struct ImageResp {
    pub image: Vec<u8>,
    pub mime_type: String,
}

pub async fn process(options: &HashMap<&str, String>) -> Result<ImageResp, String> {
    let url = options.get("url").ok_or("Missing URL")?;

    let start_fetch = Instant::now();
    let resp = reqwest::get(url)
        .await
        .map_err(|e| format!("Error fetching image: {}", e))?;
    let img_data = resp
        .bytes()
        .await
        .map_err(|e| format!("Error fetching image: {}", e))?;
    let fetch_duration = start_fetch.elapsed();
    println!("Time taken to fetch image: {:?} {:?}", url, fetch_duration);

    let start_process = Instant::now();
    let mut img = VipsImage::new_from_buffer(&img_data, "")
        .map_err(|e| format!("Error creating VipsImage: {}", e))?;

    let mut format = options.get("format").cloned().unwrap_or_default();
    if format.is_empty() {
        format = detect_mime_type(&img_data)
            .ok_or("failed to detect mime type")?
            .to_string();

        format = format.replace("image/", "");
        println!("{}", format);
    }

    if options_contains_thumbnail_param(options) {
        let (width, height) = calculate_width_and_height(&img, options);

        let opt = ThumbnailImageOptions {
            height,
            size: ops::Size::Both,
            import_profile: "sRGB".into(),
            export_profile: "sRGB".into(),
            ..ThumbnailImageOptions::default()
        };

        img = ops::thumbnail_image_with_opts(&img, width, &opt)
            .map_err(|e| format!("Error creating thumbnail image: {}", e))?;
    }

    if let Some(angle_str) = options.get("rotate") {
        let angle = angle_str
            .parse::<f64>()
            .map_err(|_| "Invalid rotate value")?;
        img = ops::rotate(&img, angle).map_err(|e| format!("Error rotating image: {}", e))?;
    }

    if options.contains_key("grayscale") {
        img = ops::colourspace(&img, ops::Interpretation::Grey16)
            .map_err(|e| format!("Error converting to grayscale: {}", e))?;
    }

    let (out, mime_type) = match format.as_str() {
        "jpeg" | "jpg" => (
            ops::jpegsave_buffer_with_opts(&img, &JpegsaveBufferOptions::default())
                .map_err(|e| format!("Error saving JPEG: {}", e))?,
            "image/jpeg".to_string(),
        ),
        "png" => (
            ops::pngsave_buffer_with_opts(&img, &PngsaveBufferOptions::default())
                .map_err(|e| format!("Error saving PNG: {}", e))?,
            "image/png".to_string(),
        ),
        "webp" => (
            ops::webpsave_buffer_with_opts(&img, &WebpsaveBufferOptions::default())
                .map_err(|e| format!("Error saving Webp: {}", e))?,
            "image/webp".to_string(),
        ),
        _ => return Err("Unsupported format".to_string()),
    };
    let process_duration = start_process.elapsed();
    println!(
        "Time taken to process image: {:?} {:?}",
        url, process_duration
    );

    Ok(ImageResp {
        image: out,
        mime_type,
    })
}

fn options_contains_thumbnail_param(options: &HashMap<&str, String>) -> bool {
    options.contains_key("width") || options.contains_key("height")
}

fn calculate_width_and_height(image: &VipsImage, options: &HashMap<&str, String>) -> (i32, i32) {
    let orig_width = image.get_width();
    let orig_height = image.get_height();
    let aspect_ratio = orig_width as f32 / orig_height as f32;

    let width = options.get("width").and_then(|w| w.parse::<i32>().ok());
    let height = options.get("height").and_then(|h| h.parse::<i32>().ok());

    match (width, height) {
        (Some(w), Some(h)) => (w, h),
        (Some(w), None) => (w, (w as f32 / aspect_ratio) as i32),
        (None, Some(h)) => ((h as f32 * aspect_ratio) as i32, h),
        (None, None) => (orig_width, orig_height),
    }
}

fn detect_mime_type(image_data: &[u8]) -> Option<&'static str> {
    infer::get(image_data).map(|info| info.mime_type())
}
