use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "provider_type")]
pub enum ObjectStorageProviderType {
    Integrated { bundles_dir: PathBuf },
    IntegratedStatic { bundles_dir: PathBuf },
}
