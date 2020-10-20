-- Your SQL goes here
CREATE TABLE chat (
    id SERIAL PRIMARY KEY,
    from_user_id INTEGER NOT NULL,
    to_user_id INTEGER NOT NULL,
    product_id INTEGER NOT NULL
);

CREATE TABLE chat_messages (
    id SERIAL PRIMARY KEY,
    chat_id INTEGER NOT NULL,
    from_user_id INTEGER NOT NULL,
    to_user_id INTEGER NOT NULL,
    msg_data TEXT NOT NULL,
    is_read BOOLEAN NOT NULL DEFAULT 'f',
    send_datetime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
