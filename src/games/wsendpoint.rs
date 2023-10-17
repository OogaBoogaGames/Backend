use axum::{http::StatusCode, Json};
use serde::Serialize;

pub async fn get_websocket_endpoint() -> (StatusCode, Json<WsEndpointResponse>) {
    (
        StatusCode::OK,
        Json(WsEndpointResponse {
            endpoint: "dssfsdsdsddssdfds".to_string(),
        }),
    )
}
#[derive(Serialize)]
pub struct WsEndpointResponse {
    endpoint: String,
}
