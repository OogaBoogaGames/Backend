use awsregion::Region;
use s3::{creds::Credentials, Bucket};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BucketDef {
    pub name: String,
    #[serde(with = "RegionDef")]
    pub region: Region,
    pub credentials: Credentials,
    pub path_style: bool,
}

impl From<BucketDef> for Bucket {
    fn from(def: BucketDef) -> Bucket {
        let mut bucket = Bucket::new(&def.name, def.region, def.credentials).unwrap();
        if def.path_style {
            bucket.set_path_style();
        }
        bucket
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(remote = "Region")]
#[serde(tag = "region_type")]
pub enum RegionDef {
    /// us-east-1
    UsEast1,
    /// us-east-2
    UsEast2,
    /// us-west-1
    UsWest1,
    /// us-west-2
    UsWest2,
    /// ca-central-1
    CaCentral1,
    /// af-south-1
    AfSouth1,
    /// ap-east-1
    ApEast1,
    /// ap-south-1
    ApSouth1,
    /// ap-northeast-1
    ApNortheast1,
    /// ap-northeast-2
    ApNortheast2,
    /// ap-northeast-3
    ApNortheast3,
    /// ap-southeast-1
    ApSoutheast1,
    /// ap-southeast-2
    ApSoutheast2,
    /// cn-north-1
    CnNorth1,
    /// cn-northwest-1
    CnNorthwest1,
    /// eu-north-1
    EuNorth1,
    /// eu-central-1
    EuCentral1,
    /// eu-central-2
    EuCentral2,
    /// eu-west-1
    EuWest1,
    /// eu-west-2
    EuWest2,
    /// eu-west-3
    EuWest3,
    /// me-south-1
    MeSouth1,
    /// sa-east-1
    SaEast1,
    /// Digital Ocean nyc3
    DoNyc3,
    /// Digital Ocean ams3
    DoAms3,
    /// Digital Ocean sgp1
    DoSgp1,
    /// Digital Ocean fra1
    DoFra1,
    /// Yandex Object Storage
    Yandex,
    /// Wasabi us-east-1
    WaUsEast1,
    /// Wasabi us-east-2
    WaUsEast2,
    /// Wasabi us-west-1
    WaUsWest1,
    /// Wasabi eu-central-1
    WaEuCentral1,
    /// Custom region
    R2 {
        account_id: String,
    },
    Custom {
        region: String,
        endpoint: String,
    },
}
