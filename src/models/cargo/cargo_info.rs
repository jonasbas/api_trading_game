use diesel::prelude::*;
use serde::Serialize;

use crate::{models::cargo::cargo_type::Cargotype, schema::cargo_info};

#[derive(Debug, Selectable, Identifiable, Queryable, Insertable, AsChangeset, Serialize)]
#[diesel(table_name = cargo_info)]
pub struct CargoInfo {
    id: i32,
    name: Cargotype,
    weight: i32,
    base_value: i32,
}
