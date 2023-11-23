mod games;
mod object_storage;
mod user;
mod util;

use axum::{response::Redirect, BoxError, Router};
use deadpool_redis::Runtime;
use object_storage::provider_base::ObjectStorageProviderType;
use rand_chacha::ChaCha8Rng;
use rand_core::{OsRng, RngCore, SeedableRng};
use scorched::{log_this, LogData, LogImportance};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use util::{
    appstate::{AppState, Argon2Config},
    id::OBGIdFactory,
};
use zbus::Connection;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    bind_address: SocketAddr,
    redis_url: String,
    object_storage_provider: ObjectStorageProviderType,
    ar2_config: Option<Argon2Config>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind_address: SocketAddr::from(([127, 0, 0, 1], 8080)),
            redis_url: "redis://localhost:6379".to_string(),
            object_storage_provider: ObjectStorageProviderType::Off,
            ar2_config: None,
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

    let redis_cfg = deadpool_redis::Config::from_url(&cfg.redis_url);
    let pool = redis_cfg.create_pool(Some(Runtime::Tokio1)).unwrap();

    let rand = ChaCha8Rng::seed_from_u64(OsRng.next_u64());

    let appstate = Arc::new(Mutex::new(AppState {
        redis: pool,
        id_factory: OBGIdFactory::new(),
        z_conn: Connection::session().await.unwrap(),
        ar2_config: cfg.ar2_config.unwrap_or_else(Default::default),
        rand,
    }));

    let app = Router::new()
        .fallback(|| async { Redirect::permanent("https://oogabooga.games/404") })
        .nest("/user", user::routes::routes(Arc::clone(&appstate)))
        .nest("/games", games::routes::routes(Arc::clone(&appstate)))
        .nest(
            "/assets",
            object_storage::routes::routes(cfg.object_storage_provider),
        )
        .layer(CorsLayer::permissive());

    log_this(LogData {
        importance: LogImportance::Info,
        message: format!("Caveman is now listening on {}", cfg.bind_address),
    })
    .await;

    axum::Server::bind(&cfg.bind_address)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
