-- Your SQL goes here
CREATE TABLE promotions (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL,
    is_marked BOOLEAN NOT NULL DEFAULT 'f',
    top_by_cat BOOLEAN NOT NULL DEFAULT 'f',
    top_by_name BOOLEAN NOT NULL DEFAULT 'f',
    is_pre_prder BOOLEAN NOT NULL DEFAULT 'f',
    prod_bought_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

