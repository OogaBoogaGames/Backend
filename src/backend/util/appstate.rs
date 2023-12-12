use deadpool_redis::Pool;

use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
// use fred::prelude::RedisClient;
use zbus::Connection;

use super::id::OBGIdFactory;

#[derive(Clone)]
pub struct AppState {
    pub redis: Pool,
    pub id_factory: OBGIdFactory,
    pub z_conn: Connection,
    pub ar2_config: Argon2Config,
    pub rand: ChaCha8Rng,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Argon2Config {
    #[serde(with = "AlgorithmDef")]
    pub algorithm: argon2::Algorithm,
    #[serde(with = "VersionDef")]
    pub version: argon2::Version,
    pub params: ParamsConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(remote = "argon2::Algorithm")]
pub enum AlgorithmDef {
    Argon2d = 0,
    Argon2i = 1,
    Argon2id = 2,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(remote = "argon2::Version")]
pub enum VersionDef {
    V0x10 = 0x10,
    V0x13 = 0x13,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ParamsConfig {
    m_cost: u32,
    t_cost: u32,
    p_cost: u32,
    output_len: Option<usize>,
}

impl TryInto<argon2::Params> for ParamsConfig {
    type Error = argon2::Error;

    fn try_into(self) -> Result<argon2::Params, Self::Error> {
        argon2::Params::new(self.m_cost, self.t_cost, self.p_cost, self.output_len)
    }
}

impl Default for Argon2Config {
    fn default() -> Self {
        let default_params = argon2::Params::default();
        Self {
            algorithm: argon2::Algorithm::Argon2id,
            version: argon2::Version::V0x13,
            params: ParamsConfig {
                m_cost: default_params.m_cost(),
                t_cost: default_params.t_cost(),
                p_cost: default_params.p_cost(),
                output_len: default_params.output_len(),
            },
        }
    }
}
