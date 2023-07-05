use diesel::prelude::*;

use crate::schema::ship_cargo;

#[derive(Debug, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = ship_cargo)]
pub struct ShipCargo {
    ship_id: i32,
    cargo_id: i32,
    amount: i32,
}
