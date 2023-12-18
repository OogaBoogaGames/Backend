use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};

use serde::Deserialize;
use tokio::sync::Mutex;

use crate::backend::{
    games::zbus::JsInterfaceProxy,
    util::{appstate::AppState, id::OBGId},
};

#[derive(Deserialize)]
pub struct CreateGame {
    pub game: String,
}

// #[debug_handler]/
pub async fn post_game(
    state: State<Arc<Mutex<AppState>>>,
    Json(payload): Json<CreateGame>,
) -> impl IntoResponse {
    let mut state = state.lock().await;

    let proxy = JsInterfaceProxy::new(&state.z_conn).await.unwrap();

    let game = OBGId::from(payload.game);

    let code = state.id_factory.generate_game();

    proxy.create_game(game.into(), code.0).await.unwrap();
    proxy.list_games().await.unwrap();
    format!("Created game {}.", game)
}
