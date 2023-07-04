use diesel::prelude::*;

use crate::schema::crew_members;

#[derive(Debug, Insertable, Identifiable, Queryable, AsChangeset)]
#[diesel(table_name = crew_members)]
pub struct CrewMember {
    id: i32,
    name: String,
}
