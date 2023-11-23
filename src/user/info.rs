use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use deadpool_redis::redis::AsyncCommands;
use serde_json::json;
use tokio::sync::Mutex;

use crate::util::appstate::AppState;

use super::user::{AuthData, User};

pub async fn post_info(
    state: State<Arc<Mutex<AppState>>>,
    Json(auth): Json<AuthData>,
) -> impl IntoResponse {
    let state = state.lock().await;

    let mut redis = state.redis.get().await.unwrap();

    let id: String = redis.get(format!("session:{}", auth.token)).await.unwrap();

    if auth.id != id {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Unauthorized"})),
        );
    }

    let result: Vec<(String, String)> = redis.hgetall(format!("user:{id}")).await.unwrap();

    let user = User::from_vec(result).await.unwrap();

    (
        StatusCode::OK,
        Json(json!({
            "username": user.username,
            "id": format!("{}", user.id),
        })),
    )
}
