use std::num::ParseIntError;

use deadpool_redis::redis::RedisError;
use serde::{Deserialize, Serialize};

use crate::util::id::OBGId;

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    // Note: Id is stored in struct as well as the redis key
    pub id: OBGId,
    // Note: Username is unique
    pub username: String,
    // Note: Password is hashed
    pub password: String,
}

impl User {
    pub fn as_tuple(&self) -> [(String, String); 3] {
        [
            ("id".to_string(), format!("{}", self.id)),
            ("username".to_string(), self.username.clone()),
            ("password".to_string(), self.password.clone()),
        ]
    }
    fn empty() -> Self {
        Self {
            id: OBGId::from(0),
            username: "Deleted User".to_string(),
            password: String::default(),
        }
    }
    pub async fn from_vec(vec: Vec<(String, String)>) -> Result<Self, ParseIntError> {
        vec.iter().fold(
            Ok(User::empty()),
            |acc, (k, v)| -> Result<Self, ParseIntError> {
                let mut acc = acc?;
                match k.as_str() {
                    "id" => acc.id = u64::from_str_radix(v, 16)?.into(),
                    "username" => acc.username = v.to_string(),
                    "password" => acc.password = v.to_string(),
                    _ => (),
                }
                Ok(acc)
            },
        )
    }
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"[private]")
            .finish()
    }
}

#[derive(Deserialize, Serialize)]
pub struct AuthData {
    pub id: String,
    pub token: String,
}
