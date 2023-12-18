use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use deadpool_redis::redis::AsyncCommands;
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;

use crate::backend::{
    games::zbus::JsInterfaceProxy,
    user::user::AuthData,
    util::{appstate::AppState, id::OBGId},
};

#[derive(Deserialize)]
pub struct CreateGame {
    pub game: String,
    pub auth: AuthData,
}

// #[debug_handler]/
pub async fn post_game(
    state: State<Arc<Mutex<AppState>>>,
    Json(payload): Json<CreateGame>,
) -> impl IntoResponse {
    let mut state = state.lock().await;

    let mut redis = state.redis.get().await.unwrap();

    let id: String = redis
        .get(format!("session:{}", payload.auth.token))
        .await
        .unwrap();

    if payload.auth.id != id {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Unauthorized"})),
        )
            .into_response();
    }

    let proxy = JsInterfaceProxy::new(&state.z_conn).await.unwrap();

    let game = OBGId::from(payload.game);

    let code = state.id_factory.generate_game();

    proxy
        .create_game(game.into(), code.0, OBGId::from(id).into())
        .await
        .unwrap();
    proxy.list_games().await.unwrap();
    Json(json!({
        "game": format!("{}", code),
    }))
    .into_response()
}
