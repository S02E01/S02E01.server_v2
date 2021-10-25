CREATE TABLE users (
    chat_id BIGINT NOT NULL PRIMARY KEY,
    hash TEXT NOT NULL,
    role INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP
);