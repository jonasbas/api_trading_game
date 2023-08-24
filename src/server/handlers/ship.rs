/*
 * Handler to list all available ship types
 * GET /ships
 */

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use diesel::{result::Error, ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;

use crate::{
    models::ship::{ship_type::ShipType, Ship, ShipToCreate},
    server::{
        db_connections::get_connection, state::ServerState,
        user_utils::get_and_authorize_player_with_id,
    },
};

use super::Response;

/*
 * Handler to list all available ship types
 * GET /ships
 * Returns a list of ShipTypes
 */
pub async fn list_ship_types(State(state): State<Arc<ServerState>>) -> Response<Vec<ShipType>> {
    use crate::schema::ship_types::dsl::*;
    let mut con = get_connection(state.db_connections.clone())?;

    let types: Result<Vec<ShipType>, Error> = ship_types.get_results(&mut con);
    if types.is_err() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error displaying available ship types".to_owned(),
        ));
    }

    Ok((StatusCode::OK, Json(types.unwrap())))
}

/*
 * Handler to buy a ship for a user
 * POST /ship/:ship_type_id/buy
 * Requires user_id, user_key and ship_name in body
 * Returns created ship
 */
#[derive(Debug, Deserialize)]
pub struct BuyShipBody {
    ship_name: String,
    id: i32,
    key: String,
}

pub async fn buy_ship_of_type(
    State(state): State<Arc<ServerState>>,
    Path(ship_type_id): Path<i32>,
    Json(body): Json<BuyShipBody>,
) -> Response<Ship> {
    let mut con = get_connection(state.db_connections.clone())?;
    let mut player = get_and_authorize_player_with_id(body.id, &mut con, body.key)?;
    let ship_to_buy: ShipType;

    {
        use crate::schema::ship_types::dsl::*;
        let ship_type: Result<ShipType, Error> =
            ship_types.filter(id.eq(ship_type_id)).first(&mut con);
        if ship_type.is_err() {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Requested ship type doesn't exist".to_owned(),
            ));
        }

        ship_to_buy = ship_type.unwrap();
    }

    let cost = ship_to_buy.cost;
    let available_money = player.money;

    if cost > available_money {
        return Err((
            StatusCode::BAD_REQUEST,
            "Player doesn't have enough money to purchase this ship".to_owned(),
        ));
    }
    let ship_to_create = ShipToCreate::new(body.ship_name, ship_type_id, player.id);
    let created_ship: Ship;

    {
        use crate::schema::ships::dsl::*;
        let queried_ship: Result<Ship, Error> = diesel::insert_into(ships)
            .values(ship_to_create)
            .get_result(&mut con);

        if queried_ship.is_err() {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error buying a new ship. Your money wasn't affected".to_owned(),
            ));
        }

        created_ship = queried_ship.unwrap();
    }

    {
        use crate::schema::players::dsl::*;
        player.money = available_money - cost;
        let update_result = diesel::update(players)
            .filter(id.eq(player.id))
            .set(player)
            .execute(&mut con);

        if update_result.is_err() {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error updating player money. The whole transaction has been aborted".to_owned(),
            ));
        }
    }

    Ok((StatusCode::CREATED, Json(created_ship)))
}
