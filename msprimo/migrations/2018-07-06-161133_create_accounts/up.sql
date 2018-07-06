CREATE TABLE accounts (
  id SERIAL PRIMARY KEY,
  username varchar NOT NULL,
  password varchar NOT NULL,
  first_name varchar NOT NULL
)