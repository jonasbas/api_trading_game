use axum::http::StatusCode;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    result::Error,
};

use diesel::prelude::*;

use crate::models::player::Player;

pub fn get_and_authorize_player_with_id(
    user_id: i32,
    con: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_key: String,
) -> Result<Player, (StatusCode, String)> {
    use crate::schema::players::dsl::*;

    let player: Result<Player, Error> = players
        .filter(id.eq(user_id))
        .select(Player::as_select())
        .first(con);

    if player.is_err() {
        return Err((StatusCode::NOT_FOUND, "Authentification error".to_owned()));
    }

    let player = player.unwrap();

    if !player.key.as_str().eq(user_key.as_str()) {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Authentification error".to_owned(),
        ));
    }

    Ok(player)
}
