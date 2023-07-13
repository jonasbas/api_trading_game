use std::time::SystemTime;

use diesel::prelude::*;
use serde::Serialize;

use crate::schema::players;

#[derive(Debug, Serialize, Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = players)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub key: String,
    pub money: i32,
    created_at: SystemTime,
}

#[derive(Insertable)]
#[diesel(table_name = players)]
pub struct PlayerToCreate {
    name: String,
    key: String,
    money: i32,
    created_at: SystemTime,
}

impl PlayerToCreate {
    pub fn new(name: String, key: String) -> Self {
        PlayerToCreate {
            name,
            key,
            money: 0,
            created_at: SystemTime::now(),
        }
    }
}

#[derive(Debug, Serialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = players)]
pub struct PlayerToDisplay {
    pub id: i32,
    pub name: String,
    money: i32,
    created_at: SystemTime,
}

#[derive(Debug, Serialize)]
pub struct CreatedPlayer {
    pub id: i32,
    pub name: String,
    pub generated_key: String,
}
