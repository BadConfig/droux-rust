-- Your SQL goes here
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    descr TEXT NOT NULL,
    price INTEGER NOT NULL,
    location VARCHAR NOT NULL,
    state VARCHAR NOT NULL,
    category_id INTEGER NOT NULL,
    seller_id INTEGER NOT NULL,
    pictures TEXT[] NOT NULL,
    create_datetime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);