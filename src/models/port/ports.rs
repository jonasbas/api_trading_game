use diesel::prelude::*;

use crate::schema::ports;

#[derive(Debug, Queryable, Insertable, Identifiable, AsChangeset)]
#[diesel(table_name = ports)]
struct Port {
    id: i32,
    name: String,
    pos_x: i32,
    pos_y: i32,
}
