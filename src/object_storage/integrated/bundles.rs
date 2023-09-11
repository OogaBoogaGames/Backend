use std::path::PathBuf;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};

pub async fn get_raw(state: State<PathBuf>, Path(package): Path<String>) -> (StatusCode, Vec<u8>) {
    match state.canonicalize() {
        Ok(base_path) => {
            let mut full_path = base_path.clone();
            full_path.push(package);

            match full_path.canonicalize() {
                Ok(full_path) => {
                    println!("{}", full_path.display());
                    // "hi" in utf-8
                    (StatusCode::OK, vec![104, 105, 10])
                }
                Err(e) => {
                    if e.raw_os_error() == Some(2) {
                        return (StatusCode::NOT_FOUND, vec![]);
                    }
                    println!("Error canonicalizing full path for asset serving: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, vec![])
                }
            }
        }
        Err(e) => {
            println!("Error canonicalizing base path for asset serving: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, vec![])
        }
    }
}
