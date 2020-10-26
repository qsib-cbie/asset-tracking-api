-- Your SQL goes here

CREATE TABLE "asset_tags"
(
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT NULL,
    serial_number VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(serial_number)
)