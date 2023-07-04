use std::io::Write;

use diesel::{deserialize::FromSql, pg::Pg, serialize::ToSql, AsExpression, FromSqlRow};

#[derive(Debug, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::schema::sql_types::Crewskill)]
pub enum CrewSkill {
    Steering,
    Trading,
    Rowing,
}

impl ToSql<crate::schema::sql_types::Crewskill, Pg> for CrewSkill {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match *self {
            CrewSkill::Steering => out.write_all(b"steering")?,
            CrewSkill::Trading => out.write_all(b"trading")?,
            CrewSkill::Rowing => out.write_all(b"rowing")?,
        }

        Ok(diesel::serialize::IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::Crewskill, Pg> for CrewSkill {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"steering" => Ok(CrewSkill::Steering),
            b"trading" => Ok(CrewSkill::Trading),
            b"rowing" => Ok(CrewSkill::Rowing),
            _ => Err("Unrecognized crew skill variant".into()),
        }
    }
}
