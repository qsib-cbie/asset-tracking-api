-- Your SQL goes here

ALTER TABLE asset_tags
ADD COLUMN asset_id BIGINT NOT NULL REFERENCES assets(id)