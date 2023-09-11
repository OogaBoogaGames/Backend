use super::provider_base::ObjectStorageProviderType;
use axum::{routing::get, Router};

pub fn routes(provider: ObjectStorageProviderType) -> Router {
    match provider {
        ObjectStorageProviderType::Integrated { bundles_dir } => Router::new()
            .route("/bundle/:package", get(super::integrated::bundles::get_raw))
            .with_state(bundles_dir),
        ObjectStorageProviderType::IntegratedStatic { bundles_dir: _ } => todo!(),
    }
}
