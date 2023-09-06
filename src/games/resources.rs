use axum::{http::StatusCode, Json};
use serde::Serialize;

pub async fn get_resources() -> (StatusCode, Json<Vec<GameResource>>) {
    (
        StatusCode::OK,
        Json(vec![GameResource {
            token: "background.image".to_string(),
            path: "examplegame/backgroundimage.png".to_string(),
            mime_type: "image/png".to_string(),
            sha256sum: "null".to_string(),
        }]),
    )
}

#[derive(Serialize)]
pub struct GameResource {
    token: String,
    path: String,
    #[serde(rename = "type")]
    mime_type: String,
    sha256sum: String,
}
