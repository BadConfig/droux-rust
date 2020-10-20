-- Your SQL goes here
CREATE TABLE activation_links (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL UNIQUE,
    reference VARCHAR NOT NULL UNIQUE
);