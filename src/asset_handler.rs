use super::utils;

use axum::{
    extract::{Path},
    response::IntoResponse,
    http::Response,
    body::{Body, StreamBody}
};
use tokio_util::io::ReaderStream;


pub async fn get_asset(Path(path): Path<String>) -> impl IntoResponse {
    let mut asset_path = "assets/".to_owned();
    asset_path.push_str(&path);

    let content_type = utils::find_mime_type(&asset_path);
    let asset = tokio::fs::read_to_string(&asset_path).await;

    if asset.is_ok() {
        Response::builder()
            .header("content-type", content_type.to_string())
            .body(asset.unwrap())
            .unwrap().into_response()
    } else {
        Response::builder()
            .body(Body::empty())
            .unwrap().into_response()
    }
}

pub async fn get_npm_asset(
    Path((asset_type, module, path)): Path<(String, String, String)>,
) -> impl IntoResponse {
    let asset_path: String;
    let content_type: String;

    match asset_type.as_str() {
        "mjs" => {
            asset_path = format!("node_modules/{}/dist/{}", module, path);
            content_type = mime::APPLICATION_JAVASCRIPT_UTF_8.to_string();
        },
        "css" => {
            asset_path = format!("node_modules/{}/build/{}", module, path);
            content_type = mime::TEXT_CSS_UTF_8.to_string();
        },
        "font" => {
            asset_path = format!("node_modules/@fontsource/{}/{}", module, path);

            if path.ends_with("woff2") {
                content_type = mime::FONT_WOFF2.to_string();
            } else if path.ends_with("woff") {
                content_type = mime::FONT_WOFF.to_string();
            } else {
                content_type = mime::TEXT_CSS_UTF_8.to_string();
            }
        }
        _ => {
            asset_path = format!("node_modules/{}/dist/{}", module, path);
            content_type = mime::APPLICATION_JAVASCRIPT_UTF_8.to_string();
        }
    }

    let file = tokio::fs::File::open(asset_path).await;

    if file.is_ok() {
        let stream = ReaderStream::new(file.unwrap());
        let body = StreamBody::new(stream);

        Response::builder()
            .header("content-type", content_type)
            .body(body)
            .unwrap().into_response()
    } else {
        Response::builder()
            .body(Body::empty())
            .unwrap().into_response()
    }
}