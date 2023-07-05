-- Your SQL goes here
CREATE TABLE ship_cargo (
  ship_id INTEGER REFERENCES ships(id),
  cargo_id INTEGER REFERENCES cargo_info(id),
  amount INTEGER NOT NULL,
  PRIMARY KEY(ship_id, cargo_id)
);
