use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use tokio::sync::Mutex;

use crate::util::appstate::AppState;

use super::{create, info, wsendpoint};

pub fn routes(state: Arc<Mutex<AppState>>) -> Router {
    Router::new()
        .route("/id/:id/info", get(info::get_info))
        .route("/create", post(create::post_game))
        .route(
            "/id/:id/player/:playerid/websocket",
            get(wsendpoint::ws_handler),
        )
        .with_state(state)
}
