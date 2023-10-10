use axum::{
    body::StreamBody,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use regex::Regex;
use s3::{error::S3Error, Bucket};

pub async fn get_raw(state: State<Bucket>, Path(package): Path<String>) -> impl IntoResponse {
    if package.contains('/') || package.contains("..") {
        return (StatusCode::FORBIDDEN, "Invalid package title.".into());
    }

    let parts: Vec<&str> = package.split('.').collect();

    let valid_part = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    if !parts.iter().all(|&part| valid_part.is_match(part)) {
        return (StatusCode::FORBIDDEN, "Invalid package title.".into());
    }

    match state.get_object(format!("{}.cvmn", package)).await {
        Ok(data) => (
            StatusCode::from_u16(data.status_code()).unwrap(),
            data.bytes().to_owned(),
        ),
        Err(err) => match err {
            S3Error::Http(status, content) => {
                (StatusCode::from_u16(status).unwrap(), content.into())
            }
            _ => (StatusCode::BAD_REQUEST, "400 Bad Request".into()),
        },
    }
}
