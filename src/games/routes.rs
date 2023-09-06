use axum::{routing::get, Router};

use super::{resources, wsendpoint};

pub fn routes() -> Router {
    Router::new()
        .route("/id/:id/resources", get(resources::get_resources))
        .route(
            "/id/:id/player/:playerid/websocket",
            get(wsendpoint::get_websocket_endpoint),
        )
}
