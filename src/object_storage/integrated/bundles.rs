use std::path::PathBuf;

use axum::{
    body::StreamBody,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use scorched::{log_this, LogData, LogImportance};
use tokio_util::io::ReaderStream;

pub async fn get_raw(state: State<PathBuf>, Path(package): Path<String>) -> impl IntoResponse {
    match state.canonicalize() {
        Ok(base_path) => {
            let mut full_path = base_path.clone();
            full_path.push(package);

            match full_path.canonicalize() {
                Ok(full_path) => {
                    if full_path.starts_with(base_path) {
                        log_this(LogData {
                            importance: LogImportance::Info,
                            message: format!("Serving bundle from path: {}", full_path.display()),
                        })
                        .await;
                        let file = match tokio::fs::File::open(full_path).await {
                            Ok(file) => file,
                            Err(_err) => {
                                return Err((
                                    StatusCode::NOT_FOUND,
                                    "Error fetching resource.".to_string(),
                                ));
                            }
                        };
                        let stream = ReaderStream::new(file);
                        let body = StreamBody::new(stream);

                        return Ok((StatusCode::OK, body));
                    }
                    Err((StatusCode::FORBIDDEN, "Forbidden directory.".to_string()))
                }
                Err(e) => {
                    if e.raw_os_error() == Some(2) {
                        return Err((StatusCode::NOT_FOUND, "Unknown resource.".to_string()));
                    }
                    log_this(LogData {
                        importance: LogImportance::Error,
                        message: format!("Error canonicalizing full path for asset serving: {}", e),
                    })
                    .await;
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Error fetching resource.".to_string(),
                    ))
                }
            }
        }
        Err(e) => {
            log_this(LogData {
                importance: LogImportance::Error,
                message: format!("Error canonicalizing base path for asset serving: {}", e),
            })
            .await;
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error fetching resource.".to_string(),
            ))
        }
    }
}
