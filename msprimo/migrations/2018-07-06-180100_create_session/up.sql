CREATE TABLE sessions (
  id serial PRIMARY KEY,
  user_id INTEGER NOT NULL,
  cookie_token VARCHAR NOT NULL
)