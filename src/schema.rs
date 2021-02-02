table! {
    alerts (id) {
        id -> Int8,
        message -> Nullable<Text>,
        reason -> Text,
        user_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    asset_scanners (id) {
        id -> Int8,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    asset_tags (id) {
        id -> Int8,
        name -> Varchar,
        description -> Nullable<Text>,
        serial_number -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    assets (id) {
        id -> Int8,
        asset_tag_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    comments (id) {
        id -> Int8,
        content -> Text,
        user_id -> Int8,
        asset_tag_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    contact_events (id) {
        id -> Int8,
        asset_tag_id -> Int8,
        location_id -> Int8,
        alert_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    locations (id) {
        id -> Int8,
        name -> Nullable<Varchar>,
        latitude -> Float4,
        longitude -> Float4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        ip -> Nullable<Inet>,
    }
}

table! {
    roles (id) {
        id -> Int8,
        name -> Varchar,
        user_id -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    rooms (id) {
        id -> Int8,
        name -> Varchar,
        location_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int8,
        username -> Text,
        token -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(alerts -> users (user_id));
joinable!(assets -> asset_tags (asset_tag_id));
joinable!(comments -> asset_tags (asset_tag_id));
joinable!(comments -> users (user_id));
joinable!(contact_events -> alerts (alert_id));
joinable!(contact_events -> asset_tags (asset_tag_id));
joinable!(contact_events -> locations (location_id));
joinable!(roles -> users (user_id));
joinable!(rooms -> locations (location_id));

allow_tables_to_appear_in_same_query!(
    alerts,
    asset_scanners,
    asset_tags,
    assets,
    comments,
    contact_events,
    locations,
    roles,
    rooms,
    users,
);
