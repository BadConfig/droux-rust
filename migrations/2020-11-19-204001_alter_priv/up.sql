-- Your SQL goes here
ALTER TABLE promotions
    ADD COLUMN in_news BOOLEAN NOT NULL DEFAULT 'f';