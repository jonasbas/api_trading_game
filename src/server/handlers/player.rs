use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use diesel::{result::Error, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;

use crate::{
    models::{
        player::{CreatedPlayer, Player, PlayerToCreate, PlayerToDisplay},
        ship::Ship,
    },
    server::{db_connections::get_connection, state::ServerState},
};

use super::Response;

#[derive(Debug, Deserialize)]
pub struct PlayerName {
    pub name: String,
}

/*
 * Handler to create a new player
 * POST /player/create
 * Returns CreatedPlayer struct
 */
pub async fn create_player(
    State(state): State<Arc<ServerState>>,
    Json(body): Json<PlayerName>,
) -> Response<CreatedPlayer> {
    use crate::schema::players;

    let key: String = rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let player_to_create = PlayerToCreate::new(body.name, key.clone());

    let mut con = get_connection(state.db_connections.clone())?;

    let created_player: Result<Player, Error> = diesel::insert_into(players::table)
        .values(player_to_create)
        .returning(Player::as_returning())
        .get_result(&mut con);

    if created_player.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let created_player = created_player.unwrap();

    Ok((
        StatusCode::CREATED,
        Json(CreatedPlayer {
            id: created_player.id,
            name: created_player.name,
            generated_key: key,
        }),
    ))
}

/*
 * Handler to display one selected player
 * GET /player/:user_id
 * Returns PlayerToDisplay
 */
pub async fn display_player(
    Path(user_id): Path<i32>,
    State(state): State<Arc<ServerState>>,
) -> Response<PlayerToDisplay> {
    use crate::schema::players::dsl::*;

    let mut con = get_connection(state.db_connections.clone())?;

    let player: Result<PlayerToDisplay, Error> = players
        .filter(id.eq(user_id))
        .select(PlayerToDisplay::as_select())
        .first(&mut con);

    if player.is_err() {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok((StatusCode::OK, Json(player.unwrap())))
}

#[derive(Debug, Deserialize)]
pub struct PlayerRename {
    name: String,
    key: String,
}

/*
 * Handler to rename selected player
 * POST /player/:user_id/rename
 * Requires player key in body
 * Returns PlayerToDisplay
 */
pub async fn rename_player(
    State(state): State<Arc<ServerState>>,
    Path(user_id): Path<i32>,
    Json(body): Json<PlayerRename>,
) -> Response<PlayerToDisplay> {
    use crate::schema::players::dsl::*;

    let mut con = get_connection(state.db_connections.clone())?;

    let player: Result<Player, Error> = players
        .filter(id.eq(user_id))
        .select(Player::as_select())
        .first(&mut con);

    if player.is_err() {
        return Err(StatusCode::NOT_FOUND);
    }

    let mut player = player.unwrap();

    if !player.key.as_str().eq(body.key.as_str()) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    player.name = body.name;

    let update_result: Result<PlayerToDisplay, Error> = diesel::update(players)
        .filter(id.eq(player.id))
        .set(player)
        .returning(PlayerToDisplay::as_returning())
        .get_result(&mut con);

    if update_result.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok((StatusCode::OK, Json(update_result.unwrap())))
}

/*
 * Handler to list all ships a player owns
 * GET /player/:id/ships
 * Returns List of Ships
 */
pub async fn list_player_ships(
    State(state): State<Arc<ServerState>>,
    Path(user_id): Path<i32>,
) -> Response<Vec<Ship>> {
    use crate::schema::ships::dsl::*;

    let mut con = get_connection(state.db_connections.clone())?;

    let all_ships: Result<Vec<Ship>, Error> =
        ships.filter(owner_id.eq(user_id)).get_results(&mut con);

    if all_ships.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok((StatusCode::OK, Json(all_ships.unwrap())))
}
