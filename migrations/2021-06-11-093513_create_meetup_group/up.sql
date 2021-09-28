CREATE TABLE meetup_groups (
  id SERIAL PRIMARY KEY,
  slug VARCHAR NOT NULL UNIQUE,
  name VARCHAR NOT NULL,
  link VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  city VARCHAR NOT NULL,
  state VARCHAR NOT NULL,
  country VARCHAR NOT NULL,
  is_private BOOLEAN NOT NULL,
  photo VARCHAR
)
