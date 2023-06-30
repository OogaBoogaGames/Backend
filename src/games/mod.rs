use axum::{
    routing::{get}, Router,
};
pub mod resources;
pub mod wsendpoint;

pub fn routes() -> Router {
    Router::new()
        .route("/id/:id/resources", get(resources::get_resources))
        .route("/id/:id/player/:playerid/websocket", get(wsendpoint::get_websocket_endpoint))
}