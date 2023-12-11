use std::{sync::Arc};

use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use deadpool_redis::redis::{AsyncCommands, RedisError};
use rand::rngs::OsRng;
// use fred::{interfaces::HashesInterface, types::RedisMap};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;


use crate::{
    user::user::User,
    util::{appstate::AppState, id::IdType},
};

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}

// #[debug_handler]
pub async fn post_user(
    state: State<Arc<Mutex<AppState>>>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let mut state = state.lock().await;

    let mut redis = state.redis.get().await.unwrap();

    let salt = SaltString::generate(&mut OsRng);

    let res: Result<String, RedisError> = redis.get(format!("lookup:{}", payload.username)).await;

    match res {
        Ok(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Username already exists"})),
            )
                .into_response()
        }
        Err(_) => {
            let ar2 = Argon2::new(
                state.ar2_config.algorithm,
                state.ar2_config.version,
                state.ar2_config.params.clone().try_into().unwrap(),
            );

            let hash = ar2
                .hash_password(payload.password.as_bytes(), &salt)
                .unwrap();

            let id = state.id_factory.generate(IdType::User);

            let user = User {
                id,
                username: payload.username.clone(),
                password: hash.to_string(),
            };

            let _: () = redis
                .hset_multiple(format!("user:{}", id.to_string()), &user.as_tuple())
                .await
                .unwrap();

            let _: () = redis
                .set(format!("lookup:{}", payload.username), format!("{}", id))
                .await
                .unwrap();

            (
                StatusCode::CREATED,
                Json(json!({
                    "id": format!("{}", id),
                    "username": payload.username,
                })),
            )
                .into_response()
        }
    }
}
