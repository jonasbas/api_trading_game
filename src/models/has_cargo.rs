use diesel::prelude::*;

use crate::schema::has_cargo;

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = has_cargo)]
pub struct HasCargo {
    port_id: i32,
    cargo_id: i32,
}
