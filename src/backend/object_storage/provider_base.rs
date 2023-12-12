use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[cfg(feature = "storage-s3")]
use super::s3::def::BucketDef;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "provider_type")]
pub enum ObjectStorageProviderType {
    Off,
    #[cfg(feature = "storage-integrated")]
    Integrated {
        bundles_dir: PathBuf,
    },
    #[cfg(feature = "storage-s3")]
    S3 {
        bucket: BucketDef,
    },
}
