use diesel::prelude::*;

use crate::schema::crew_members;

#[derive(Debug, Insertable, Identifiable, AsChangeset)]
#[diesel(table_name = crew_members)]
pub struct CrewMember {
    id: i32,
    name: String,
}
