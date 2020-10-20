-- Your SQL goes here
CREATE TABLE brands(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE
);

INSERT INTO brands (id, name) VALUES 
(0,'my brand1'),
(1,'my brand2'),
(2,'my brand3');