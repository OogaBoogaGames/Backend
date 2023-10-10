use super::provider_base::ObjectStorageProviderType;
use axum::{
    routing::{any, get},
    Router,
};

pub fn routes(provider: ObjectStorageProviderType) -> Router {
    match provider {
        ObjectStorageProviderType::Off => Router::new().fallback(any(|| async {
            "Object storage is disabled. Is there a reverse proxy?"
        })),
        #[cfg(feature = "storage-integrated")]
        ObjectStorageProviderType::Integrated { bundles_dir } => Router::new()
            .route("/bundle/:package", get(super::integrated::bundles::get_raw))
            .with_state(bundles_dir),
        #[cfg(feature = "storage-s3")]
        ObjectStorageProviderType::S3 { bucket } => Router::new()
            .route("/bundle/:package", get(super::s3::bundles::get_raw))
            .with_state(bucket.into()),
    }
}
