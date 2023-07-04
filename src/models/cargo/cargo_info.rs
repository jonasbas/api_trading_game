use diesel::prelude::*;

use crate::{models::cargo::cargo_type::Cargotype, schema::cargo_info};

#[derive(Debug, Identifiable, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = cargo_info)]
pub struct CargoInfo {
    id: i32,
    name: Cargotype,
    weight: i32,
    base_value: i32,
}
