use std::time::SystemTime;

use diesel::prelude::*;

use crate::schema::players;

#[derive(Debug, Insertable, Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = players)]
pub struct Player {
    id: i32,
    name: String,
    key: String,
    money: i32,
    created_at: SystemTime,
}
