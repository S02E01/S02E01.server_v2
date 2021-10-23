CREATE TABLE users (
    chat_id BIGINT NOT NULL PRIMARY KEY,
    user_role INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP
);