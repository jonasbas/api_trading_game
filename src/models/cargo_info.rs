use diesel::{AsChangeset, Identifiable, Insertable, Queryable};

use crate::{models::cargo_type::Cargotype, schema::cargo_info};

#[derive(Debug, Identifiable, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = cargo_info)]
pub struct CargoInfo {
    id: i32,
    name: Cargotype,
    weight: i32,
    base_value: i32,
}
