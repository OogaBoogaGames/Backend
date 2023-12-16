use axum::{http::Method, response::Redirect, Router};
use backend::object_storage::provider_base::ObjectStorageProviderType;
use backend::util::{
    appstate::{AppState, Argon2Config},
    id::OBGIdFactory,
};
use deadpool_redis::Runtime;
use oogaboogagames_backend::backend;
use rand_chacha::ChaCha8Rng;
use rand_core::{OsRng, RngCore, SeedableRng};
use scorched::{logf, LogData, LogImportance};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
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

    logf!(
        Info,
        "Loading config file from {}",
        confy::get_configuration_file_path("oogaboogagames-backend", None)?.display()
    );

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

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any);

    let app = Router::new()
        .fallback(|| async { Redirect::permanent("https://oogabooga.games/404") })
        .nest(
            "/user",
            backend::user::routes::routes(Arc::clone(&appstate)),
        )
        .nest(
            "/games",
            backend::games::routes::routes(Arc::clone(&appstate)),
        )
        .nest(
            "/assets",
            backend::object_storage::routes::routes(cfg.object_storage_provider),
        )
        .layer(ServiceBuilder::new().layer(cors));
    logf!(Info, "Backend is now listening on {}", cfg.bind_address);

    let listener = TcpListener::bind(cfg.bind_address).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
