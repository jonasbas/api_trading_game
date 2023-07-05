use diesel::prelude::*;

use crate::schema::crew_to_ship;

#[derive(Debug, Insertable, Queryable)]
#[diesel(table_name = crew_to_ship)]
pub struct CrewToShip {
    crew_id: i32,
    ship_id: i32,
}
