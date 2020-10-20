-- Your SQL goes here
CREATE TABLE rating (
    id SERIAL PRIMARY KEY,
    voter_id INTEGER NOT NULL,
    seller_id INTEGER NOT NULL,
    stars SMALLINT NOT NULL
);