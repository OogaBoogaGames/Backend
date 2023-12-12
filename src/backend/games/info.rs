use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_macros::debug_handler;
use serde::Serialize;
use tokio::sync::Mutex;

use crate::backend::util::appstate::AppState;

#[debug_handler]
pub async fn get_info(
    _state: State<Arc<Mutex<AppState>>>,
    Path(_id): Path<u64>,
) -> impl IntoResponse {
    (StatusCode::OK, format!("{:#?}", "TODO"))
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
