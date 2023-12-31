-- Your SQL goes here
CREATE TABLE ships (
  id INTEGER GENERATED BY DEFAULT AS IDENTITY UNIQUE PRIMARY KEY,
  name VARCHAR NOT NULL,
  pos_x INTEGER NOT NULL,
  pos_y INTEGER NOT NULL,
  ship_type_id INTEGER NOT NULL REFERENCES ship_types(id) ON DELETE CASCADE,
  owner_id INTEGER NOT NULL REFERENCES players(id) ON DELETE CASCADE,
  created_at TIMESTAMP NOT NULL
);
