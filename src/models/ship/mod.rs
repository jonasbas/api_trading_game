mod ship_cargo;
mod ship_type;

use std::time::SystemTime;

use diesel::prelude::*;

use crate::schema::ships;

#[derive(Debug, Insertable, Identifiable, Queryable, AsChangeset)]
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
