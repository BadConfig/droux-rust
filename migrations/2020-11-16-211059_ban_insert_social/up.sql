-- Your SQL goes here
ALTER TABLE users
    ADD COLUMN is_banned BOOL NOT NULL DEFAULT 'f';

DELETE FROM social_links;

INSERT INTO social_links (id,name,link,icon) VALUES
(1,'facebook',	'https://facebook.com',	'/static/assets/facebook.svg'),
(2, 'instagram', 'https://instagram.com', '/static/assets/instagram.svg'),
(3, 'youtube', 'https://youtube.com', '/static/assets/youtube.svg'),
(4, 'vk', 'https://vk.com', '/static/assets/vk.svg'),
(5, 'telegram', 'https://telegram.me', '/static/assets/telegram.svg');