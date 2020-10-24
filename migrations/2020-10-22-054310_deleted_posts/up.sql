-- Your SQL goes here
CREATE TABLE deleted_posts (
    id SERIAL NOT NULL,
    post_id INTEGER NOT NULL UNIQUE
);