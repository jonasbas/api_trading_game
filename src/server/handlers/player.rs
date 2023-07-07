use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use rand::{distributions::Alphanumeric, Rng};

use crate::{
    models::player::player::{CreatedPlayer, Player, PlayerToCreate, PlayerToDisplay},
    server::state::ServerState,
};

/*
 * Handler to create a new player
 * Returns CreatedPlayer struct
 */
pub async fn create_player(State(state): State<Arc<ServerState>>) -> Json<CreatedPlayer> {
    use crate::schema::players;

    let key: String = rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let player_to_create = PlayerToCreate::new("Jack Sparrow".into(), key.clone());

    let mut con = state
        .db_connections
        .get()
        .expect("Couldn't get db connection");

    let created_player: Player = diesel::insert_into(players::table)
        .values(player_to_create)
        .returning(Player::as_returning())
        .get_result(&mut con)
        .expect("Error creating player.");

    Json(CreatedPlayer {
        id: created_player.id,
        name: created_player.name,
        generated_key: key,
    })
}

/*
 * Handler to display one selected player
 * Accepts user_id as i32
 * Returns PlayerToDisplay
 */
pub async fn display_player(
    Path(user_id): Path<i32>,
    State(state): State<Arc<ServerState>>,
) -> Json<PlayerToDisplay> {
    use crate::schema::players::dsl::*;

    let mut con = state
        .db_connections
        .get()
        .expect("Couldn't get db connection");

    let player: PlayerToDisplay = players
        .filter(id.eq(user_id))
        .select(PlayerToDisplay::as_select())
        .first(&mut con)
        .expect("");

    Json(player)
}
