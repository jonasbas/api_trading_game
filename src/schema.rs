// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "cargotype"))]
    pub struct Cargotype;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Cargotype;

    cargo_info (id) {
        id -> Int4,
        name -> Cargotype,
        weight -> Int4,
        base_value -> Int4,
    }
}
