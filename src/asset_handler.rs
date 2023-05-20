use super::utils;

use axum::{
    extract::{Path},
    response::IntoResponse,
    http::Response,
    body::Body
};


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