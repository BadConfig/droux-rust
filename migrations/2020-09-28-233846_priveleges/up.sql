-- Your SQL goes here
CREATE TABLE priveleges (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL UNIQUE,
    privelege_type VARCHAR NOT NULL
);

INSERT INTO priveleges (user_id,privelege_type) VALUES (0, 'admin');
INSERT INTO priveleges (user_id,privelege_type) VALUES (1, 'moderator');
INSERT INTO priveleges (user_id,privelege_type) VALUES (2, 'editor');