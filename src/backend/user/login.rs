use std::{sync::Arc, u128};

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_macros::debug_handler;
use deadpool_redis::{
    redis::{AsyncCommands, RedisError},
    Connection,
};
use rand_chacha::ChaCha8Rng;
use rand_core::RngCore;
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Mutex;

use crate::backend::{user::user::User, util::appstate::AppState};

use super::user::AuthData;

async fn new_session(
    random: &mut ChaCha8Rng,
    id: String,
    redis: &mut Connection,
) -> Result<u128, RedisError> {
    let mut u128_pool = [0u8; 16];
    random.fill_bytes(&mut u128_pool);

    let token = u128::from_le_bytes(u128_pool);

    let _: () = redis.set(format!("session:{:X}", token), id).await.unwrap();

    Ok(token)
}

#[derive(Deserialize)]
pub struct LoginCredentials {
    username: String,
    password: String,
}

#[debug_handler]
pub async fn post_login(
    state: State<Arc<Mutex<AppState>>>,
    Json(creds): Json<LoginCredentials>,
) -> impl IntoResponse {
    let mut state = state.lock().await;

    let mut redis = state.redis.get().await.unwrap();

    let ar2 = Argon2::new(
        state.ar2_config.algorithm,
        state.ar2_config.version,
        state.ar2_config.params.clone().try_into().unwrap(),
    );

    let id: String = redis
        .get(format!("lookup:{}", creds.username))
        .await
        .unwrap();

    let result: Vec<(String, String)> = redis.hgetall(format!("user:{id}")).await.unwrap();

    let user = User::from_vec(result).await.unwrap();

    let authenticated = ar2
        .verify_password(
            creds.password.as_bytes(),
            &PasswordHash::new(&user.password).unwrap(),
        )
        .ok()
        .is_some();

    match authenticated {
        true => {
            let token = new_session(&mut state.rand, id.clone(), &mut redis)
                .await
                .unwrap();
            (
                StatusCode::OK,
                Json(AuthData {
                    id,
                    token: format!("{:X}", token),
                }),
            )
                .into_response()
        }
        false => (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid credentials"})),
        )
            .into_response(),
    }
}
