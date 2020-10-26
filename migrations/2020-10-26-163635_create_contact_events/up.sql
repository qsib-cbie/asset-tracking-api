-- Your SQL goes here

CREATE TABLE contact_events
(
    id BIGSERIAL PRIMARY KEY,
    asset_tag_id BIGINT NOT NULL REFERENCES asset_tags(id),
    location_id BIGINT NOT NULL REFERENCES locations(id),
    alert_id BIGINT NULL REFERENCES alerts(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)