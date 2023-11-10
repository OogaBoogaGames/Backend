mod games;
mod object_storage;
mod util;

use axum::{http::Method, response::Redirect, Router};
use fred::{
    prelude::{ClientLike, RedisClient},
    types::{Builder, RedisConfig},
};
use object_storage::provider_base::{self, ObjectStorageProviderType};
use scorched::{log_this, LogData, LogImportance};
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use util::{appstate::AppState, id::OBGIdFactory};
use zbus::Connection;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    bind_address: SocketAddr,
    redis_url: String,
    object_storage_provider: provider_base::ObjectStorageProviderType,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind_address: SocketAddr::from(([127, 0, 0, 1], 8080)),
            redis_url: "redis://localhost:6379".to_string(),
            object_storage_provider: ObjectStorageProviderType::Off,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), confy::ConfyError> {
    tracing_subscriber::fmt::init();

    log_this(LogData {
        importance: LogImportance::Debug,
        message: format!(
            "Loading config file from {}",
            confy::get_configuration_file_path("oogaboogagames-backend", None)?.display()
        ),
    })
    .await;

    let cfg: Config = confy::load("oogaboogagames-backend", None)?;

    let client = Builder::from_config(RedisConfig::from_url(&cfg.redis_url).unwrap())
        .build()
        .unwrap();
    let redis_connect_task = client.connect();
    client.wait_for_connect().await.unwrap();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let appstate = AppState {
        redis: client,
        id_factory: OBGIdFactory::new(),
        z_conn: Connection::session().await.unwrap(),
    };

    let app = Router::new()
        .fallback(|| async { Redirect::permanent("https://oogabooga.games/404") })
        .nest("/games", games::routes::routes(appstate))
        .nest(
            "/assets",
            object_storage::routes::routes(cfg.object_storage_provider),
        )
        .layer(ServiceBuilder::new().layer(cors));

    log_this(LogData {
        importance: LogImportance::Info,
        message: format!("Caveman is now listening on {}", cfg.bind_address),
    })
    .await;

    axum::Server::bind(&cfg.bind_address)
        .serve(app.into_make_service())
        .await
        .unwrap();
    let _ = redis_connect_task.await;
    Ok(())
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
