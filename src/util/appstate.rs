use fred::prelude::RedisClient;
use zbus::Connection;

use super::id::OBGIdFactory;

#[derive(Clone)]
pub struct AppState {
    pub redis: RedisClient,
    pub id_factory: OBGIdFactory,
    pub z_conn: Connection,
}
