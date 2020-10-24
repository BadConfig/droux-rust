-- Your SQL goes here
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE
);

INSERT INTO categories (name) VALUES
('Сумки'),
('Обувь'),
('Верхняя одежда');