CREATE TABLE users(
    id 				SERIAL 		PRIMARY KEY,
    email 			VARCHAR 	NOT NULL UNIQUE,
    username 		VARCHAR 	NOT NULL UNIQUE,
    pass_hash 		VARCHAR 	NOT NULL,
    picture_path 	VARCHAR 	NOT NULL DEFAULT 'default_picture.png',
    verifyed 		BOOLEAN 	NOT NULL DEFAULT 'f',
    rate_summ 		BIGINT 		NOT NULL DEFAULT 0,
    rate_count 		BIGINT 		NOT NULL DEFAULT 0,
    register_data 	TIMESTAMP 	NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- категории товаров
CREATE TABLE categories (
    id 		SERIAL PRIMARY KEY,
    name 	VARCHAR NOT NULL UNIQUE
);

-- подкатегории (связаны с категориями и с товаром)
CREATE TABLE sub_categories (
    id 			SERIAL PRIMARY KEY,
    category_id INTEGER REFERENCES categories ON DELETE CASCADE NOT NULL,
    name 		VARCHAR NOT NULL UNIQUE
);


-- Состояние продукта (описание: поношенное новое итд)
CREATE TABLE product_state (
    id 		SERIAL 	PRIMARY KEY,
    name 	VARCHAR NOT NULL UNIQUE
);

-- список всех брендов для продукта
CREATE TABLE brands(
    id 		SERIAL 	PRIMARY KEY,
    name 	VARCHAR NOT NULL UNIQUE
);

-- м/ж
CREATE TABLE product_type (
    id 		SERIAL 	PRIMARY KEY,
    name 	VARCHAR NOT NULL
);

CREATE TABLE sizes (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE
);

CREATE TABLE products (
    id 				SERIAL 		PRIMARY KEY,
    sub_category_id INTEGER 	REFERENCES sub_categories ON DELETE CASCADE NOT NULL,
    title 			VARCHAR 	NOT NULL,
    descr 			TEXT		NOT NULL,
    price 			INTEGER 	NOT NULL,
    location 		VARCHAR 	NOT NULL,
    product_state 	INTEGER 	REFERENCES product_state ON DELETE CASCADE NOT NULL,
    brand_id 		INTEGER 	REFERENCES brands ON DELETE CASCADE NOT NULL,
    seller_id 		INTEGER 	REFERENCES users ON DELETE CASCADE NOT NULL,
    pictures 		TEXT[] 		NOT NULL,
    is_published 	BOOLEAN		NOT NULL DEFAULT 'f',
    type_id         INTEGER     REFERENCES product_type ON DELETE CASCADE NOT NULL,
    size_id         INTEGER     REFERENCES sizes ON DELETE CASCADE NOT NULL,
    total_views 	BIGINT 		NOT NULL DEFAULT 0,
    create_datetime TIMESTAMP	NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE promotions (
    id 					SERIAL 		PRIMARY KEY,
    product_id 			INTEGER 	REFERENCES products ON DELETE CASCADE NOT NULL,
    is_marked 			BOOLEAN 	NOT NULL DEFAULT 'f',
    top_by_cat 			BOOLEAN 	NOT NULL DEFAULT 'f',
    top_by_name 		BOOLEAN 	NOT NULL DEFAULT 'f',
    is_pre_prder 		BOOLEAN 	NOT NULL DEFAULT 'f',
    prod_bought_date 	TIMESTAMP 	NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE deleted_posts (
    id 		SERIAL 	PRIMARY KEY,
    post_id INTEGER REFERENCES products ON DELETE CASCADE UNIQUE NOT NULL
);

CREATE TABLE views (
    id 			SERIAL 	PRIMARY KEY,
    product_id 	INTEGER REFERENCES products ON DELETE CASCADE UNIQUE NOT NULL,
    count 		INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE favourites (
    id 			SERIAL 	PRIMARY KEY,
    user_id 	INTEGER REFERENCES users ON DELETE CASCADE NOT NULL,
    product_id 	INTEGER REFERENCES products ON DELETE CASCADE NOT NULL
);

CREATE TABLE rating (
    id 			SERIAL PRIMARY KEY,
    voter_id 	INTEGER REFERENCES users ON DELETE CASCADE NOT NULL,
    seller_id 	INTEGER REFERENCES users ON DELETE CASCADE NOT NULL,
    stars 		SMALLINT NOT NULL,
    comment 	VARCHAR NOT NULL
);


CREATE TABLE activation_links (
    id 			SERIAL PRIMARY KEY,
    user_id 	INTEGER REFERENCES users ON DELETE CASCADE NOT NULL,
    reference 	VARCHAR NOT NULL UNIQUE
);

CREATE TABLE subscribes (
    id 		SERIAL 	PRIMARY KEY,
    from_id INTEGER REFERENCES users ON DELETE CASCADE NOT NULL,
    to_id 	INTEGER REFERENCES users ON DELETE CASCADE NOT NULL
);

CREATE TABLE priveleges (
    id 				SERIAL 	PRIMARY KEY,
    user_id 		INTEGER REFERENCES users ON DELETE CASCADE NOT NULL,
    privelege_type 	VARCHAR NOT NULL
);

CREATE TABLE chat (
    id 				SERIAL 	PRIMARY KEY,
    from_user_id 	INTEGER REFERENCES users ON DELETE CASCADE NOT NULL,
    to_user_id 		INTEGER REFERENCES users ON DELETE CASCADE NOT NULL,
    product_id 		INTEGER NOT NULL
);

CREATE TABLE chat_messages (
    id 				SERIAL 		PRIMARY KEY,
    chat_id 		INTEGER 	REFERENCES chat ON DELETE CASCADE NOT NULL,
    from_user_id 	INTEGER 	REFERENCES users NOT NULL,
    to_user_id 		INTEGER 	REFERENCES users NOT NULL,
    msg_data 		TEXT 		NOT NULL,
    is_read 		BOOLEAN 	NOT NULL DEFAULT 'f',
    send_datetime 	TIMESTAMP 	NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users (id,email,username,pass_hash,verifyed) VALUES (
    	1,
    	'array.clean@gmail.com',
    	'artemij_admin',
    	'EDC65C457EB4D5921CFF5505D4870F91B5A1F0F95F1AF706DD7A1E51AE68B3BB',
    	't'
	),
	(
    	2,
    	'exit.lab@mail.ru',
    	'artemij_moderator',
    	'81AE196CDA0530F3A9B589A7D34BD6380262937933122E2B8E14392CF8315335',
    	't'
	),
	(
    	3,
    	'editor@editor.ru',
    	'artemij_editor',
    	'543698FC86A8A9C6C43F9BC0FA1723545E070F80D1D3E8EFB1DF6E53332116C2',
    	't'
);

INSERT INTO priveleges (user_id,privelege_type) VALUES 
	(1, 'admin'),
	(2, 'moderator'),
	(3, 'editor');

INSERT INTO product_state (id,name) VALUES 
    (1,'Новое с биркой'),
    (2,'Новое без бирки'),
    (3,'Немного ношенное'),
    (4,'Сильно ношенное');
    
INSERT INTO categories (id, name) VALUES
	(1,'Сумки'),
	(2,'Обувь'),
	(3,'Верхняя одежда');
   
INSERT INTO sub_categories (category_id,name) VALUES 
	(1, 'подкат1'),
	(1, 'подкат2'),
	(1, 'подкат3'),
	(2, 'подкат4'),
	(2, 'подкат5');

INSERT INTO brands (id, name) VALUES 
	(1,'my brand1'),
	(2,'my brand2'),
	(3,'my brand3');

INSERT INTO sizes (id,name) VALUES
	(1,'36US'),
	(2,'36.5US'),
	(3,'37US'),
	(4,'37.5US'),
	(5,'38US'),
	(6,'38.5US'),
	(7,'39US'),
	(8,'39.5US'),
	(9,'40US'),
	(10,'40.5US');

INSERT INTO product_type (id,name) VALUES
	(1,'мужчины'),
	(2,'женьщины');
	
CREATE OR REPLACE FUNCTION proccess_user_rating() RETURNS TRIGGER AS $$
DECLARE

BEGIN
    IF    TG_OP = 'INSERT' THEN
        UPDATE users SET 
        	rate_summ = rate_summ + NEW.stars, 
        	rate_count = rate_count + 1
        WHERE NEW.seller_id = users.id;
        RETURN NEW;
    ELSIF TG_OP = 'UPDATE' THEN
        UPDATE users SET 	
        	rate_summ = rate_summ - OLD.stars + NEW.stars
        WHERE NEW.seller_id = users.id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE users SET 	
        	rate_summ = rate_summ - OLD.stars, 
        	rate_count = rate_count - 1
        WHERE OLD.seller_id = users.id;
        RETURN OLD;
    END IF;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_user_rating
AFTER INSERT OR UPDATE OR DELETE ON rating FOR EACH ROW EXECUTE PROCEDURE proccess_user_rating ();