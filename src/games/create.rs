use axum::{extract::State, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use zbus::dbus_proxy;

use crate::util::{
    appstate::AppState,
    id::{GameId, IdType, OBGId},
};

#[derive(Deserialize)]
pub struct CreateGame {
    pub game_type: String,
}

#[dbus_proxy(
    interface = "games.oogabooga.JsHost",
    default_service = "games.oogabooga.JsHost",
    default_path = "/games/oogabooga/JsHost"
)]
trait MyGreeter {
    async fn say_hello(&self, name: &str) -> zbus::Result<String>;
}

pub async fn post_game(
    mut state: State<AppState>,
    Json(payload): Json<CreateGame>,
) -> impl IntoResponse {
    let id = state.id_factory.generate(IdType::Game);

    let proxy = MyGreeterProxy::new(&state.z_conn).await.unwrap();

    proxy.say_hello(&id.to_string()).await.unwrap()
}
