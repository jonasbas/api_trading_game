use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use diesel::{result::Error, RunQueryDsl};

use crate::{
    models::cargo::cargo_info::CargoInfo,
    server::{db_connections::get_connection, state::ServerState},
};

use super::Response;

/*
 * Handler to list all available cargo types
 * GET /cargo
 */
pub async fn list_cargo_infos(State(state): State<Arc<ServerState>>) -> Response<Vec<CargoInfo>> {
    use crate::schema::cargo_info::dsl::*;

    let mut con = get_connection(state.db_connections.clone())?;
    let all_cargo: Result<Vec<CargoInfo>, Error> = cargo_info.get_results(&mut con);

    if all_cargo.is_err() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error displaying cargo types".to_owned(),
        ));
    }

    Ok((StatusCode::OK, Json(all_cargo.unwrap())))
}
