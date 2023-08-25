use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use diesel::{result::Error, RunQueryDsl};

use crate::{
    models::port::ports::Port,
    server::{db_connections::get_connection, state::ServerState},
};

use super::Response;

/*
 * Handler to display all available ports
 * GET /ports
 */
pub async fn list_all_ports(State(state): State<Arc<ServerState>>) -> Response<Vec<Port>> {
    use crate::schema::ports::dsl::*;

    let mut con = get_connection(state.db_connections.clone())?;
    let all_ports: Result<Vec<Port>, Error> = ports.get_results(&mut con);

    if all_ports.is_err() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error displaying all available ports".to_owned(),
        ));
    }

    Ok((StatusCode::OK, Json(all_ports.unwrap())))
}
