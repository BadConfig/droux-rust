-- Your SQL goes here
ALTER TABLE rating 
    ADD COLUMN feedback_type VARCHAR NOT NULL;

ALTER TABLE deleted_posts 
    ADD COLUMN delete_type VARCHAR NOT NULL;