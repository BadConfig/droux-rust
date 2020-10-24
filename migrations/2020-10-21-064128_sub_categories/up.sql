-- Your SQL goes here
CREATE TABLE sub_categories (
    id SERIAL PRIMARY KEY,
    category_id INTEGER NOT NULL,
    name VARCHAR NOT NULL
);

INSERT INTO sub_categories (category_id,name) VALUES 
(1, 'подкат1'),
(1, 'подкат2'),
(1, 'подкат3'),
(2, 'подкат4'),
(2, 'подкат5');