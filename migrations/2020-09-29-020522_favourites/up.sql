-- Your SQL goes here
CREATE TABLE favourites (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    product_id INTEGER NOT NULL
)