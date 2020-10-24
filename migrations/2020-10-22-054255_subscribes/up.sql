-- Your SQL goes here
CREATE TABLE subscribes (
    id SERIAL PRIMARY KEY,
    from_id INTEGER NOT NULL,
    to_id INTEGER NOT NULL
);