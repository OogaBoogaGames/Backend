use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_macros::debug_handler;
use serde::Serialize;

use crate::util::{appstate::AppState, id::OBGId};

#[debug_handler]
pub async fn get_info(state: State<AppState>, Path(id): Path<u64>) -> impl IntoResponse {
    (StatusCode::OK, format!("{:#?}", OBGId::from(id)))
    // match state.hgetall(format!("id:{}", id)).await {
    //     Ok(data) => (StatusCode::NOT_FOUND, format!("{:?}", data)),
    //     Err(err) => {
    //         println!("{}", err);
    //         (StatusCode::NOT_FOUND, "Unknown game id".to_string())
    //     }
    // }
}

#[derive(Serialize)]
pub struct GameInfo {
    id: String,
    s: String,
    #[serde(rename = "type")]
    mime_type: String,
    sha256sum: String,
}
