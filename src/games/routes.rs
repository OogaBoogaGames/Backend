use axum::{
    routing::{get, post},
    Router,
};
use fred::prelude::RedisClient;

use crate::util::appstate::AppState;

use super::{create, info, wsendpoint};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/id/:id/info", get(info::get_info))
        .route("/create", post(create::post_game))
        .route(
            "/id/:id/player/:playerid/websocket",
            get(wsendpoint::ws_handler),
        )
        .with_state(state)
}
