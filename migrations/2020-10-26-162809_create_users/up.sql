-- Your SQL goes here

CREATE TABLE users
(
    id BIGSERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    token TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)