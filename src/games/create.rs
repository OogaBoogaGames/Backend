use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Extension, Json};
use axum_macros::debug_handler;
use serde::Deserialize;
use tokio::sync::Mutex;
use zbus::dbus_proxy;

use crate::{
    games::zbus::JsInterfaceProxy,
    util::{
        appstate::AppState,
        id::{GameId, IdType, OBGId},
    },
};

#[derive(Deserialize)]
pub struct CreateGame {
    pub game_type: String,
}

// #[debug_handler]/
pub async fn post_game(
    state: State<Arc<Mutex<AppState>>>,
    Json(payload): Json<CreateGame>,
) -> impl IntoResponse {
    let mut state = state.lock().await;

    let id = state.id_factory.generate(IdType::Game);

    let proxy = JsInterfaceProxy::new(&state.z_conn).await.unwrap();

    proxy.create_game(id.into()).await.unwrap();
    format!("Created game {}.", id.to_string())
}
