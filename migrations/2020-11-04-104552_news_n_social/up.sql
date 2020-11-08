-- Your SQL goes here
CREATE TABLE social_links (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    link VARCHAR NOT NULL UNIQUE,
    icon VARCHAR NOT NULL UNIQUE
);

INSERT INTO social_links (name,link,icon) VALUES 
('telegram','https://telegram.me','telegram'),
('vk','https://vk.com','vk');

CREATE TABLE news (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL UNIQUE,
    body TEXT NOT NULL,
    picture VARCHAR NOT NULL UNIQUE,
    creation_datetime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO news (title,body,picture) VALUES 
('article 1','article text','article_pic'),
('article 2','article text2','picture');
