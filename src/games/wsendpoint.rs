use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Serialize;

pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            format!("echo: {:?}", msg.into_data()).into()
        } else {
            // client disconnected
            return;
        };

        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}

// Arc::new(Mutex::new(JsRuntime::new(RuntimeOptions {
//     extensions: vec![js::ext::oogabooga::init_ops_and_esm()],
//     ..Default::default()
// }))),
