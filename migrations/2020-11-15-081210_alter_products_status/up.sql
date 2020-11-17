-- Your SQL goes here
DROP TABLE deleted_posts;

ALTER TABLE products
    DROP COLUMN is_published;

ALTER TABLE products
    ADD COLUMN status VARCHAR NOT NULL DEFAULT 'on_watch';

-- on_watch
-- published
-- deleted
-- refused
-- sold