CREATE TABLE users (
    id BIGSERIAL NOT NULL,
    chat_id BIGINT NOT NULL PRIMARY KEY,
    user_role INT NOT NULL,
    create_at TEXT NOT NULL
);