-- Your SQL goes here
CREATE TABLE has_cargo (
  port_id INTEGER REFERENCES ports(id) ON DELETE CASCADE,
  cargo_id INTEGER REFERENCES cargo_info(id) ON DELETE CASCADE,
  PRIMARY KEY(port_id, cargo_id)
);
