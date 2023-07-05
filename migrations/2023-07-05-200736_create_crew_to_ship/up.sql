-- Your SQL goes here
CREATE TABLE crew_to_ship (
  crew_id INTEGER REFERENCES crew_members(id),
  ship_id INTEGER REFERENCES ships(id),
  PRIMARY KEY (crew_id, ship_id)
);
