/*
 * Handler to list all available ship types
 * GET /ships
 */

use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use diesel::{result::Error, RunQueryDsl};

use crate::{
    models::ship::ship_type::ShipType,
    server::{db_connections::get_connection, state::ServerState},
};

use super::Response;

pub async fn list_ship_types(State(state): State<Arc<ServerState>>) -> Response<Vec<ShipType>> {
    use crate::schema::ship_types::dsl::*;
    let mut con = get_connection(state.db_connections.clone())?;

    let types: Result<Vec<ShipType>, Error> = ship_types.get_results(&mut con);
    if types.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok((StatusCode::OK, Json(types.unwrap())))
}
