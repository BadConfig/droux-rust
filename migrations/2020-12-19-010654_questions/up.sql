-- Your SQL goes here
CREATE TABLE rescue_issues (
    id SERIAL PRIMARY KEY UNIQUE,
    contact VARCHAR NOT NULL,
    issue VARCHAR NOT NULL
);