use diesel::prelude::*;

use crate::schema::crew_member_skills;

use super::crew_skill::CrewSkill;

#[derive(Debug, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = crew_member_skills)]
pub struct CrewMemberSkill {
    crew_id: i32,
    skill: CrewSkill,
    value: i32,
}
