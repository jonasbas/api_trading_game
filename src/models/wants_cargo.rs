use diesel::prelude::*;

use crate::schema::wants_cargo;

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = wants_cargo)]
pub struct WantsCargo {
    port_id: i32,
    cargo_id: i32,
}
