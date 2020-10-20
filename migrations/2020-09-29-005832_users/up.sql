-- Your SQL goes here
CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL UNIQUE,
    username VARCHAR NOT NULL UNIQUE,
    pass_hash VARCHAR NOT NULL,
    picture_path VARCHAR NOT NULL DEFAULT '/default_picture',
    verifyed BOOLEAN NOT NULL DEFAULT 'f',
    rate_summ BIGINT NOT NULL DEFAULT 0,
    rate_count BIGINT NOT NULL DEFAULT 0,
    register_data TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users (id,email,username,pass_hash,verifyed) VALUES (
    0,
    'array.clean@gmail.com',
    'artemij_admin',
    'EDC65C457EB4D5921CFF5505D4870F91B5A1F0F95F1AF706DD7A1E51AE68B3BB',
    't'
);

INSERT INTO users (id,email,username,pass_hash,verifyed) VALUES (
    1,
    'exit.lab@mail.ru',
    'artemij_moderator',
    '81AE196CDA0530F3A9B589A7D34BD6380262937933122E2B8E14392CF8315335',
    't'
);

INSERT INTO users (id,email,username,pass_hash,verifyed) VALUES (
    2,
    'editor@editor.ru',
    'artemij_editor',
    '543698FC86A8A9C6C43F9BC0FA1723545E070F80D1D3E8EFB1DF6E53332116C2',
    't'
);