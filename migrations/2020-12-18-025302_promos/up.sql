-- Your SQL goes here
CREATE TABLE promos (
    id SERIAL PRIMARY KEY,
    promo VARCHAR NOT NULL UNIQUE,
    sale INTEGER NOT NULL
);