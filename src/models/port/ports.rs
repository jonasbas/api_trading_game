use diesel::prelude::*;
use serde::Serialize;

use crate::schema::ports;

#[derive(Debug, Queryable, Insertable, Identifiable, AsChangeset, Serialize)]
#[diesel(table_name = ports)]
pub struct Port {
    id: i32,
    name: String,
    pos_x: i32,
    pos_y: i32,
}
