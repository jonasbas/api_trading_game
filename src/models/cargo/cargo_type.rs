use std::io::Write;

use diesel::{
    deserialize::FromSql,
    pg::Pg,
    serialize::{IsNull, ToSql},
    AsExpression, FromSqlRow,
};
use serde::Serialize;

#[derive(Debug, AsExpression, FromSqlRow, Serialize)]
#[diesel(sql_type = crate::schema::sql_types::Cargotype)]
pub enum Cargotype {
    Wood,
    Wool,
    Rum,
}

impl ToSql<crate::schema::sql_types::Cargotype, Pg> for Cargotype {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match *self {
            Cargotype::Wood => out.write_all(b"wood")?,
            Cargotype::Wool => out.write_all(b"wool")?,
            Cargotype::Rum => out.write_all(b"rum")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::Cargotype, Pg> for Cargotype {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"wood" => Ok(Cargotype::Wood),
            b"wool" => Ok(Cargotype::Wool),
            b"rum" => Ok(Cargotype::Rum),
            _ => Err("Unrecognized cargo variant".into()),
        }
    }
}
