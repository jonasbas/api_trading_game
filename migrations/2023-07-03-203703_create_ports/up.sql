-- Your SQL goes here
CREATE TABLE ports (
  id INTEGER GENERATED BY DEFAULT AS IDENTITY UNIQUE PRIMARY KEY,
  name VARCHAR NOT NULL,
  pos_x INTEGER NOT NULL,
  pos_y INTEGER NOT NULL
);
