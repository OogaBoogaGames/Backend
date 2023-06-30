mod games;

use axum::{
    routing::{get}, Router,
};
use std::{net::SocketAddr};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .nest("/games", games::routes());


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
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


