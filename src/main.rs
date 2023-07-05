mod games;

use axum::{response::Redirect, routing::get, Router};
use scorched::{log_this, LogData, LogImportance};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .fallback(|| async { Redirect::permanent("https://oogabooga.games/404") })
        .nest("/games", games::routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    log_this(LogData {
        importance: LogImportance::Info,
        message: format!("Caveman is now listening on {}", addr),
    });
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// async fn create_user(
//     Json(payload): Json<CreateUser>,
// ) -> (StatusCode, Json<User>) {
//     let user = User {
//         id: 1337,
//         username: payload.username,
//     };

//     (StatusCode::CREATED, Json(user))
// }

// #[derive(Deserialize)]
// struct CreateUser {
//     username: String,
// }

// #[derive(Serialize)]
// struct User {
//     id: u64,
//     username: String,
// }
