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
    crew_to_ship (crew_id, ship_id) {
        crew_id -> Int4,
        ship_id -> Int4,
    }
}

diesel::table! {
    has_cargo (port_id, cargo_id) {
        port_id -> Int4,
        cargo_id -> Int4,
    }
}

diesel::table! {
    players (id) {
        id -> Int4,
        name -> Varchar,
        key -> Varchar,
        money -> Int4,
        created_at -> Timestamp,
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
    ship_cargo (ship_id, cargo_id) {
        ship_id -> Int4,
        cargo_id -> Int4,
        amount -> Int4,
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
    ships (id) {
        id -> Int4,
        name -> Varchar,
        pos_x -> Int4,
        pos_y -> Int4,
        ship_type_id -> Int4,
        owner_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    wants_cargo (port_id, cargo_id) {
        port_id -> Int4,
        cargo_id -> Int4,
    }
}

diesel::joinable!(crew_member_skills -> crew_members (crew_id));
diesel::joinable!(crew_to_ship -> crew_members (crew_id));
diesel::joinable!(crew_to_ship -> ships (ship_id));
diesel::joinable!(has_cargo -> cargo_info (cargo_id));
diesel::joinable!(has_cargo -> ports (port_id));
diesel::joinable!(ship_cargo -> cargo_info (cargo_id));
diesel::joinable!(ship_cargo -> ships (ship_id));
diesel::joinable!(ships -> players (owner_id));
diesel::joinable!(ships -> ship_types (ship_type_id));
diesel::joinable!(wants_cargo -> cargo_info (cargo_id));
diesel::joinable!(wants_cargo -> ports (port_id));

diesel::allow_tables_to_appear_in_same_query!(
    cargo_info,
    crew_member_skills,
    crew_members,
    crew_to_ship,
    has_cargo,
    players,
    ports,
    ship_cargo,
    ship_types,
    ships,
    wants_cargo,
);
