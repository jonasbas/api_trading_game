pub mod ship_cargo;
pub mod ship_type;

use std::time::SystemTime;

use diesel::prelude::*;
use serde::Serialize;

use crate::schema::ships;

#[derive(Debug, Serialize, Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = ships)]
pub struct Ship {
    id: i32,
    name: String,
    pos_x: i32,
    pos_y: i32,
    ship_type_id: i32,
    owner_id: i32,
    created_at: SystemTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = ships)]
pub struct ShipToCreate {
    name: String,
    pos_x: i32,
    pos_y: i32,
    ship_type_id: i32,
    owner_id: i32,
    created_at: SystemTime,
}

impl ShipToCreate {
    pub fn new(name: String, ship_type_id: i32, owner_id: i32) -> Self {
        ShipToCreate {
            name,
            pos_x: 0,
            pos_y: 0,
            ship_type_id,
            owner_id,
            created_at: SystemTime::now(),
        }
    }
}
