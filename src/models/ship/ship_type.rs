use diesel::prelude::*;
use serde::Serialize;

use crate::schema::ship_types;

#[derive(Debug, Insertable, Queryable, Identifiable, AsChangeset, Serialize)]
#[diesel(table_name = ship_types)]
pub struct ShipType {
    pub id: i32,
    pub name: String,
    pub speed: i32,
    pub max_cargo_size: i32,
    pub max_crew_size: i32,
    pub cost: i32,
}
