// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "cargotype"))]
    pub struct Cargotype;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "crewskill"))]
    pub struct Crewskill;
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

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Crewskill;

    crew_member_skills (crew_id, skill) {
        crew_id -> Int4,
        skill -> Crewskill,
        value -> Int4,
    }
}

diesel::table! {
    crew_members (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    has_cargo (port_id, cargo_id) {
        port_id -> Int4,
        cargo_id -> Int4,
    }
}

diesel::table! {
    ports (id) {
        id -> Int4,
        name -> Varchar,
        pos_x -> Int4,
        pos_y -> Int4,
    }
}

diesel::table! {
    ship_types (id) {
        id -> Int4,
        name -> Varchar,
        speed -> Int4,
        max_cargo_size -> Int4,
        max_crew_size -> Int4,
        cost -> Int4,
    }
}

diesel::table! {
    wants_cargo (port_id, cargo_id) {
        port_id -> Int4,
        cargo_id -> Int4,
    }
}

diesel::joinable!(crew_member_skills -> crew_members (crew_id));
diesel::joinable!(has_cargo -> cargo_info (cargo_id));
diesel::joinable!(has_cargo -> ports (port_id));
diesel::joinable!(wants_cargo -> cargo_info (cargo_id));
diesel::joinable!(wants_cargo -> ports (port_id));

diesel::allow_tables_to_appear_in_same_query!(
    cargo_info,
    crew_member_skills,
    crew_members,
    has_cargo,
    ports,
    ship_types,
    wants_cargo,
);
