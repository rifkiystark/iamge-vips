# A Simple Image Processing Service

An HTTP service for image manipulation, and resizing using Rust with Axum and libvips. This service allows you to fetch images from a URL, apply transformations (such as resizing, rotating, and grayscale conversion), and return the processed image in different formats.

## Features

-   Resize images while maintaining aspect ratio.
-   Rotate images to a specified angle.
-   Convert images to grayscale.
-   Support for JPEG, PNG, and WebP formats.

## Requirements

-   Rust (latest stable version recommended)
-   `tokio` for async execution
-   `axum` for the HTTP server
-   `reqwest` for fetching images
-   `libvips` for image processing
-   `serde` for parsing request parameters

## Installation

1. Clone this repository:

    ```sh
    git clone https://github.com/rifkiystark/iamge-vips.git
    cd iamge-vips
    ```

2. Install dependencies:

    ```sh
    cargo build
    ```

3. Run the server:
    ```sh
    cargo run
    ```

## API Endpoints

### `GET /process`

Processes an image based on query parameters and returns the manipulated image.

#### Query Parameters:

| Parameter   | Type   | Description                                      |
| ----------- | ------ | ------------------------------------------------ |
| `url`       | String | URL of the image to process (Required)           |
| `width`     | Int    | Resize width (Optional)                          |
| `height`    | Int    | Resize height (Optional)                         |
| `format`    | String | Output format (`jpeg`, `png`, `webp`) (Optional) |
| `rotate`    | Float  | Rotate image by degrees (Optional)               |
| `grayscale` | Bool   | Convert image to grayscale (Optional)            |

#### Example Request:

```sh
curl "http://localhost:3000/process?url=https://example.com/image.jpg&width=500&height=500&format=png"
```

## Contributing

Feel free to submit pull requests to improve the service.

## License

MIT License
