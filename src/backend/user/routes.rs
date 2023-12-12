use std::sync::Arc;

use axum::{routing::post, Router};
use tokio::sync::Mutex;

use crate::backend::util::appstate::AppState;

use super::{create, info, login};

pub fn routes(state: Arc<Mutex<AppState>>) -> Router {
    Router::new()
        .route("/create", post(create::post_user))
        .route("/login", post(login::post_login))
        .route("/info", post(info::post_info))
        .with_state(state)
}
