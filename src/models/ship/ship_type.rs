use diesel::prelude::*;
use serde::Serialize;

use crate::schema::ship_types;

#[derive(Debug, Insertable, Queryable, Identifiable, AsChangeset, Serialize)]
#[diesel(table_name = ship_types)]
pub struct ShipType {
    id: i32,
    name: String,
    speed: i32,
    max_cargo_size: i32,
    max_crew_size: i32,
    cost: i32,
}
