use axum::{http::StatusCode, Json};

pub mod cargo;
pub mod player;
pub mod ship;

pub type Response<T> = Result<(StatusCode, Json<T>), (StatusCode, String)>;
