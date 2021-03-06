#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_service::Service;
use actix_web::middleware::Logger;
use actix_web::{dev::ServiceRequest, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;

use http::header;

use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod auth;
mod db;
mod error_handler;
mod schema;

mod alerts;
mod asset_scanners;
mod asset_tags;
mod assets;
mod comments;
mod contact_events;
mod health;
mod locations;
mod roles;
mod rooms;
mod users;

macro_rules! AppFactory {
    () => {
        || {
            App::new()
                .wrap(Logger::default())
                .wrap(HttpAuthentication::bearer(auth::validator))
                .wrap_fn(|req, srv| {
                    let mut req: ServiceRequest = req.into();
                    let headers = req.headers_mut();
                    if !headers.contains_key("authorization") {
                        headers.insert(
                            header::HeaderName::from_static("authorization"),
                            header::HeaderValue::from_static("Bearer _"),
                        )
                    }

                    srv.call(req)
                })
                .configure(alerts::init_routes)
                .configure(asset_tags::init_routes)
                .configure(asset_scanners::init_routes)
                .configure(assets::init_routes)
                .configure(comments::init_routes)
                .configure(contact_events::init_routes)
                .configure(health::init_routes)
                .configure(roles::init_routes)
                .configure(rooms::init_routes)
                .configure(users::init_routes)
                .configure(locations::init_routes)
        }
    };
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    db::init();
    auth::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(AppFactory!());

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};
    use ipnetwork::IpNetwork;
    use lazy_static::lazy_static;
    use serde::{Deserialize, Serialize};
    use std::convert::TryInto;

    lazy_static! {
        static ref FIXTURE: () = {
            dotenv().ok();
            env_logger::init();
            db::init();
            auth::init();
            ()
        };
        static ref ADMIN_USER: users::AuthUser = {
            let user = users::User::create(users::MaybeUser {
                username: "admin".into(),
                password: "qsib".into(),
            })
            .expect("Failed to create test admin user");
            user.try_into().expect("Failed to create auth user")
        };

                //create initial asset for asset tag test
                static ref INITIAL_ASSET: assets::Asset = {
                    let asset = assets::Asset::create(assets::MaybeAsset {
                        asset_tag_id: None,
                        deleted: false
                    })
                    .expect("Failed to create test asset");
                    asset.try_into().expect("Failed to create initial asset")
                };

                // create initial asset tag for comment test
                static ref INITIAL_ASSET_TAG: asset_tags::AssetTag = {
                    let asset_tag = asset_tags::AssetTag::create(asset_tags::MaybeAssetTag {
                        name: String::from("initial"),
                        description: Some(String::from("inital")),
                        serial_number: String::from("initial"),
                        asset_id: Some(INITIAL_ASSET.id),
                        deleted: false
                    })
                    .expect("Failed to create test asset tag");
                    asset_tag.try_into().expect("Failed to create initial asset tag")
                };

                // create initial location for room and contact_event test
                static ref INITIAL_LOCATION: locations::Location = {
                    let location = locations::Location::create(locations::MaybeLocation {
                        name: Some(String::from("initial")),
                        latitude: 1.0,
                        longitude: 1.0,
                        ip: Some(IpNetwork::V4("10.9.0.32/16".parse().unwrap())),
                    })
                    .expect("Failed to create test location");
                    location.try_into().expect("Failed to create initial location")
                };

                // create initial alert for contact_event test
                // uses ADMIN USER for user association
                static ref INITIAL_ALERT: alerts::Alert = {
                    let alert = alerts::Alert::create(alerts::MaybeAlert {
                        message: Some(String::from("initial")),
                        reason: String::from("initial"),
                        user_id: ADMIN_USER.id
                    })
                    .expect("Failed to create test alert");
                    alert.try_into().expect("Failed to create initial alert")
                };
    }

    pub fn setup() {
        lazy_static::initialize(&FIXTURE);
        lazy_static::initialize(&ADMIN_USER);
        lazy_static::initialize(&INITIAL_ASSET_TAG);
        lazy_static::initialize(&INITIAL_LOCATION);
        lazy_static::initialize(&INITIAL_ALERT);
    }

    #[derive(Serialize, Deserialize)]
    struct Empty {}

    #[actix_rt::test]
    async fn test_health_get_without_token() {
        setup();
        log::info!("Token: {:?}", ADMIN_USER.token);

        let mut app = test::init_service(AppFactory!()()).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let _resp = test::read_response(&mut app, req).await;
    }

    #[actix_rt::test]
    async fn test_create_and_use_user() {
        setup();

        let mut app = test::init_service(AppFactory!()()).await;

        let user = users::MaybeUser {
            username: String::from("foo"),
            password: String::from("secretpassword"),
        };
        let payload = serde_json::to_string(&user).expect("Invalid value");

        let req = test::TestRequest::post()
            .uri("/users")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();
        let resp: users::AuthUser = test::read_response_json(&mut app, req).await;
        log::info!("Created User: {:?}", resp);

        let req = test::TestRequest::get()
            .uri("/asset_tags")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", resp.token).as_str(),
            )
            .to_request();
        let _protected_resp: Vec<asset_tags::AssetTag> =
            test::read_response_json(&mut app, req).await;
    }

    #[actix_rt::test]
    async fn test_create_and_login_with_user() {
        setup();

        let mut app = test::init_service(AppFactory!()()).await;

        let user = users::MaybeUser {
            username: String::from("foo"),
            password: String::from("secretpassword"),
        };
        let payload = serde_json::to_string(&user).expect("Invalid value");

        let req = test::TestRequest::post()
            .uri("/users")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();
        let resp: users::AuthUser = test::read_response_json(&mut app, req).await;
        log::info!("Created User: {:?}", resp);

        let token = resp.token;
        let payload = serde_json::to_string(&user).expect("Invalid value");

        let req = test::TestRequest::post()
            .uri("/login")
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();
        let resp: users::AuthUser = test::read_response_json(&mut app, req).await;
        assert_eq!(token, resp.token);

        // bad password
        let bad_user = users::MaybeUser {
            username: String::from("foo"),
            password: String::from("WRONGPASSWORD"),
        };
        let payload = serde_json::to_string(&bad_user).expect("Invalid value");

        let req = test::TestRequest::post()
            .uri("/login")
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn test_user_cant_change_other_users() {
        setup();

        let mut app = test::init_service(AppFactory!()()).await;

        // Create user1
        let maybe_user = users::MaybeUser {
            username: String::from("user1"),
            password: String::from("secretpassword"),
        };
        let payload = serde_json::to_string(&maybe_user).expect("Invalid value");

        let req = test::TestRequest::post()
            .uri("/users")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();
        let user1: users::AuthUser = test::read_response_json(&mut app, req).await;
        log::info!("Created User: {:?}", user1);

        // Change user1's password as user1
        let maybe_user = users::MaybeUser {
            username: String::from("user1"),
            password: String::from("newsecretpassword"),
        };
        let payload = serde_json::to_string(&maybe_user).expect("Invalid value");

        let req = test::TestRequest::put()
            .uri(format!("/users/{}", user1.id).as_str())
            .header(header::AUTHORIZATION, format!("Bearer {}", user1.token))
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();
        let user1: users::AuthUser = test::read_response_json(&mut app, req).await;
        log::info!("Updated User: {:?}", user1);

        // Use user1's new token
        let req = test::TestRequest::get()
            .uri("/asset_tags")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", user1.token).as_str(),
            )
            .to_request();
        let _protected_resp: Vec<asset_tags::AssetTag> =
            test::read_response_json(&mut app, req).await;

        // Create user2
        let maybe_user = users::MaybeUser {
            username: String::from("user2"),
            password: String::from("secretpassword"),
        };
        let payload = serde_json::to_string(&maybe_user).expect("Invalid value");

        let req = test::TestRequest::post()
            .uri("/users")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();
        let user2: users::AuthUser = test::read_response_json(&mut app, req).await;
        log::info!("Created User: {:?}", user2);

        // Fail to change user1's password
        let payload = serde_json::to_string(&maybe_user).expect("Invalid value");

        let req = test::TestRequest::put()
            .uri(format!("/users/{}", user1.id).as_str())
            .header(header::AUTHORIZATION, format!("Bearer {}", user2.token))
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

        // Use user1's token
        let req = test::TestRequest::get()
            .uri("/asset_tags")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", user1.token).as_str(),
            )
            .to_request();
        let _protected_resp: Vec<asset_tags::AssetTag> =
            test::read_response_json(&mut app, req).await;

        // Use user2's token
        let req = test::TestRequest::get()
            .uri("/asset_tags")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", user2.token).as_str(),
            )
            .to_request();
        let _protected_resp: Vec<asset_tags::AssetTag> =
            test::read_response_json(&mut app, req).await;
    }

    #[actix_rt::test]
    async fn test_asset_tags_resource() {
        setup();

        // Find all tags, there should only be the initial one
        let mut app = test::init_service(AppFactory!()()).await;
        let req = test::TestRequest::get()
            .uri("/asset_tags")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<asset_tags::AssetTag> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 1);

        // Create a tag
        let value = asset_tags::MaybeAssetTag {
            name: String::from("foo"),
            description: Some(String::from("bar")),
            serial_number: String::from("asdf"),
            asset_id: Some(INITIAL_ASSET.id),
            deleted: false,
        };
        let payload = serde_json::to_string(&value).expect("Invalid value");

        let req = test::TestRequest::post()
            .uri("/asset_tags")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();
        let resp: asset_tags::AssetTag = test::read_response_json(&mut app, req).await;
        assert_eq!(value.name, resp.name);
        assert_eq!(value.description, resp.description);
        assert_eq!(value.serial_number, resp.serial_number);
        assert_eq!(value.asset_id, resp.asset_id);
        assert_eq!(value.deleted, resp.deleted);

        let id = resp.id;

        // Find all tags, it should include the one we just created
        let req = test::TestRequest::get()
            .uri("/asset_tags")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<asset_tags::AssetTag> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 2);
        assert_eq!(value.name, resp[1].name);
        assert_eq!(value.description, resp[1].description);
        assert_eq!(value.serial_number, resp[1].serial_number);
        assert_eq!(value.asset_id, resp[1].asset_id);
        assert_eq!(value.deleted, resp[1].deleted);

        // Create another tag
        let another_value = asset_tags::MaybeAssetTag {
            name: String::from("foo1"),
            description: Some(String::from("asdflkj")),
            serial_number: String::from("asdf1"),
            asset_id: Some(INITIAL_ASSET.id),
            deleted: false,
        };
        let payload = serde_json::to_string(&another_value).expect("Invalid value");

        let req = test::TestRequest::post()
            .uri("/asset_tags")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();
        let resp: asset_tags::AssetTag = test::read_response_json(&mut app, req).await;
        assert_eq!(another_value.name, resp.name);
        assert_eq!(another_value.description, resp.description);
        assert_eq!(another_value.serial_number, resp.serial_number);
        assert_eq!(another_value.asset_id, resp.asset_id);
        assert_eq!(another_value.deleted, resp.deleted);

        // Find all tags, it should include the two we just created
        let req = test::TestRequest::get()
            .uri("/asset_tags")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<asset_tags::AssetTag> = test::read_response_json(&mut app, req).await;

        // This order is not guaranteed by the endpoint. It is an undefined side effect of the underlying postgres query.
        assert_eq!(resp.len(), 3);
        assert_eq!(value.name, resp[1].name);
        assert_eq!(value.description, resp[1].description);
        assert_eq!(value.serial_number, resp[1].serial_number);
        assert_eq!(value.asset_id, resp[1].asset_id);
        assert_eq!(value.deleted, resp[1].deleted);
        assert_eq!(another_value.name, resp[2].name);
        assert_eq!(another_value.description, resp[2].description);
        assert_eq!(another_value.serial_number, resp[2].serial_number);
        assert_eq!(another_value.asset_id, resp[2].asset_id);
        assert_eq!(another_value.deleted, resp[2].deleted);

        // Delete first asset_tag
        let req = test::TestRequest::delete()
            .uri(format!("/asset_tags/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: asset_tags::AssetTag = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.deleted, true);

        // Find all asset_tags, there should only be 2
        let req = test::TestRequest::get()
            .uri("/asset_tags")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<asset_tags::AssetTag> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 2);

        // Find all deleted asset_tags, there should be the deleted one
        let req = test::TestRequest::get()
            .uri("/asset_tags/deleted")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<asset_tags::AssetTag> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 1);
        assert_eq!(id, resp[0].id);
        assert_eq!(resp[0].deleted, true);

        // Find all asset_tags including deleted ones, there should be 3
        let req = test::TestRequest::get()
            .uri("/asset_tags/all")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<asset_tags::AssetTag> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 3);
    }

    #[actix_rt::test]
    async fn test_assets_resource() {
        setup();

        // Find all assets, there should be the initial one
        let mut app = test::init_service(AppFactory!()()).await;
        let req = test::TestRequest::get()
            .uri("/assets")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<assets::Asset> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 1);

        // Create a asset with INITIAL ASSET TAG as asset_tag association
        let value = assets::MaybeAsset {
            asset_tag_id: Some(INITIAL_ASSET_TAG.id),
            deleted: false,
        };
        let payload = serde_json::to_string(&value).expect("Invalid value");

        let req = test::TestRequest::post()
            .uri("/assets")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();
        let resp: assets::Asset = test::read_response_json(&mut app, req).await;
        assert_eq!(value.asset_tag_id, resp.asset_tag_id);
        assert_eq!(value.deleted, resp.deleted);

        // Find all assets, it include the one we just created
        let req = test::TestRequest::get()
            .uri("/assets")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<assets::Asset> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 2);
        assert_eq!(value.asset_tag_id, resp[1].asset_tag_id);
        assert_eq!(value.deleted, resp[1].deleted);

        // Find asset by id
        let id = resp[1].id;

        let req = test::TestRequest::get()
            .uri(format!("/assets/id/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: assets::Asset = test::read_response_json(&mut app, req).await;
        assert_eq!(id, resp.id);
        assert_eq!(value.asset_tag_id, resp.asset_tag_id);
        assert_eq!(value.deleted, resp.deleted);

        // Update asset by id
        let value_updated = assets::MaybeAsset {
            asset_tag_id: Some(INITIAL_ASSET_TAG.id),
            deleted: false,
        };
        let payload_updated = serde_json::to_string(&value_updated).expect("Invalid value");

        let req = test::TestRequest::put()
            .uri(format!("/assets/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload_updated)
            .to_request();
        let resp: assets::Asset = test::read_response_json(&mut app, req).await;
        assert_eq!(value_updated.asset_tag_id, resp.asset_tag_id);
        assert_eq!(value_updated.deleted, resp.deleted);

        // Find asset by id, should be the updated one
        let req = test::TestRequest::get()
            .uri(format!("/assets/id/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: assets::Asset = test::read_response_json(&mut app, req).await;
        assert_eq!(id, resp.id);
        assert_eq!(value_updated.asset_tag_id, resp.asset_tag_id);
        assert_eq!(value_updated.deleted, resp.deleted);

        // Delete the asset by id
        let req = test::TestRequest::delete()
            .uri(format!("/assets/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: assets::Asset = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.deleted, true);

        // Find all assets, there should only be the initial one
        let req = test::TestRequest::get()
            .uri("/assets")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<assets::Asset> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 1);

        // Find all deleted assets, there should be the deleted one
        let req = test::TestRequest::get()
            .uri("/assets/deleted")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<assets::Asset> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 1);
        assert_eq!(id, resp[0].id);
        assert_eq!(resp[0].deleted, true);

        // Find all assets including deleted ones, there should be 2
        let req = test::TestRequest::get()
            .uri("/assets/all")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<assets::Asset> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 2);
    }

    #[actix_rt::test]
    async fn test_role_resource() {
        setup();

        // Find all roles, there should be none
        let mut app = test::init_service(AppFactory!()()).await;
        let req = test::TestRequest::get()
            .uri("/roles")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<roles::Role> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 0);

        // Create a role with ADMIN USER as user association
        let value = roles::MaybeRole {
            name: String::from("foo"),
            user_id: Some(ADMIN_USER.id),
        };
        let payload = serde_json::to_string(&value).expect("Invalid value");

        let req = test::TestRequest::post()
            .uri("/roles")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();
        let resp: roles::Role = test::read_response_json(&mut app, req).await;
        assert_eq!(value.name, resp.name);
        assert_eq!(value.user_id, resp.user_id);

        // Find all roles, it should be the one we just created
        let req = test::TestRequest::get()
            .uri("/roles")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<roles::Role> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 1);
        assert_eq!(value.name, resp[0].name);
        assert_eq!(value.user_id, resp[0].user_id);

        // Find role by id
        let id = resp[0].id;

        let req = test::TestRequest::get()
            .uri(format!("/roles/id/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: roles::Role = test::read_response_json(&mut app, req).await;
        assert_eq!(id, resp.id);
        assert_eq!(value.name, resp.name);
        assert_eq!(value.user_id, resp.user_id);

        // Update role by id
        let value_updated = roles::MaybeRole {
            name: String::from("foobar"),
            user_id: Some(ADMIN_USER.id),
        };
        let payload_updated = serde_json::to_string(&value_updated).expect("Invalid value");

        let req = test::TestRequest::put()
            .uri(format!("/roles/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload_updated)
            .to_request();
        let resp: roles::Role = test::read_response_json(&mut app, req).await;
        assert_eq!(value_updated.name, resp.name);
        assert_eq!(value_updated.user_id, resp.user_id);

        // Find role by id, should be the updated one
        let req = test::TestRequest::get()
            .uri(format!("/roles/id/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: roles::Role = test::read_response_json(&mut app, req).await;
        assert_eq!(id, resp.id);
        assert_eq!(value_updated.name, resp.name);
        assert_eq!(value_updated.user_id, resp.user_id);

        // Delete the role by id
        let req = test::TestRequest::delete()
            .uri(format!("/roles/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: usize = test::read_response_json(&mut app, req).await;
        assert_eq!(1, resp);

        // Find all roles, there should be none now
        let req = test::TestRequest::get()
            .uri("/roles")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<roles::Role> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 0);
    }

    #[actix_rt::test]
    async fn test_asset_scanner_resource() {
        setup();

        // Find all scanners, there should be none
        let mut app = test::init_service(AppFactory!()()).await;
        let req = test::TestRequest::get()
            .uri("/asset_scanners")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<asset_scanners::AssetScanner> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 0);

        // Create a scanner
        let value = asset_scanners::MaybeAssetScanner {
            name: String::from("foo"),
        };
        let payload = serde_json::to_string(&value).expect("Invalid value");

        let req = test::TestRequest::post()
            .uri("/asset_scanners")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();
        let resp: asset_scanners::AssetScanner = test::read_response_json(&mut app, req).await;
        assert_eq!(value.name, resp.name);

        // Find all scanners, it should be the one we just created
        let req = test::TestRequest::get()
            .uri("/asset_scanners")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<asset_scanners::AssetScanner> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 1);
        assert_eq!(value.name, resp[0].name);

        // Find scanner by id
        let id = resp[0].id;

        let req = test::TestRequest::get()
            .uri(format!("/asset_scanners/id/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: asset_scanners::AssetScanner = test::read_response_json(&mut app, req).await;
        assert_eq!(id, resp.id);
        assert_eq!(value.name, resp.name);

        // Update scanner by id
        let value_updated = asset_scanners::MaybeAssetScanner {
            name: String::from("foobar"),
        };
        let payload_updated = serde_json::to_string(&value_updated).expect("Invalid value");

        let req = test::TestRequest::put()
            .uri(format!("/asset_scanners/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload_updated)
            .to_request();
        let resp: asset_scanners::AssetScanner = test::read_response_json(&mut app, req).await;
        assert_eq!(value_updated.name, resp.name);

        // Find scanner by id, should be the updated one
        let req = test::TestRequest::get()
            .uri(format!("/asset_scanners/id/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: asset_scanners::AssetScanner = test::read_response_json(&mut app, req).await;
        assert_eq!(id, resp.id);
        assert_eq!(value_updated.name, resp.name);

        // Delete the scanner by id
        let req = test::TestRequest::delete()
            .uri(format!("/asset_scanners/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: usize = test::read_response_json(&mut app, req).await;
        assert_eq!(1, resp);

        // Find all scanners, there should be none now
        let req = test::TestRequest::get()
            .uri("/asset_scanners")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<asset_scanners::AssetScanner> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 0);
    }

    #[actix_rt::test]
    async fn test_comment_resource() {
        setup();

        // Find all comments, there should be none
        let mut app = test::init_service(AppFactory!()()).await;
        let req = test::TestRequest::get()
            .uri("/comments")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<comments::Comment> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 0);

        // Create a comment with ADMIN USER and INITIAL ASSET TAG associations
        let value = comments::MaybeComment {
            content: String::from("foo"),
            user_id: ADMIN_USER.id,
            asset_tag_id: INITIAL_ASSET_TAG.id,
        };
        let payload = serde_json::to_string(&value).expect("Invalid value");

        let req = test::TestRequest::post()
            .uri("/comments")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();
        let resp: comments::Comment = test::read_response_json(&mut app, req).await;
        assert_eq!(value.content, resp.content);
        assert_eq!(value.user_id, resp.user_id);
        assert_eq!(value.asset_tag_id, resp.asset_tag_id);

        // Find all comments, it should be the one we just created
        let req = test::TestRequest::get()
            .uri("/comments")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<comments::Comment> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 1);
        assert_eq!(value.content, resp[0].content);
        assert_eq!(value.user_id, resp[0].user_id);
        assert_eq!(value.asset_tag_id, resp[0].asset_tag_id);

        // Find comment by id
        let id = resp[0].id;

        let req = test::TestRequest::get()
            .uri(format!("/comments/id/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: comments::Comment = test::read_response_json(&mut app, req).await;
        assert_eq!(id, resp.id);
        assert_eq!(value.content, resp.content);
        assert_eq!(value.user_id, resp.user_id);
        assert_eq!(value.asset_tag_id, resp.asset_tag_id);

        // Update comment by id
        let value_updated = comments::MaybeComment {
            content: String::from("foobar"),
            user_id: ADMIN_USER.id,
            asset_tag_id: INITIAL_ASSET_TAG.id,
        };
        let payload_updated = serde_json::to_string(&value_updated).expect("Invalid value");

        let req = test::TestRequest::put()
            .uri(format!("/comments/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload_updated)
            .to_request();
        let resp: comments::Comment = test::read_response_json(&mut app, req).await;
        assert_eq!(value_updated.content, resp.content);
        assert_eq!(value_updated.user_id, resp.user_id);
        assert_eq!(value_updated.asset_tag_id, resp.asset_tag_id);

        // Find comment by id, should be the updated one
        let req = test::TestRequest::get()
            .uri(format!("/comments/id/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: comments::Comment = test::read_response_json(&mut app, req).await;
        assert_eq!(id, resp.id);
        assert_eq!(value_updated.content, resp.content);
        assert_eq!(value_updated.user_id, resp.user_id);
        assert_eq!(value_updated.asset_tag_id, resp.asset_tag_id);

        // Delete the comment by id
        let req = test::TestRequest::delete()
            .uri(format!("/comments/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: usize = test::read_response_json(&mut app, req).await;
        assert_eq!(1, resp);

        // Find all comments, there should be none now
        let req = test::TestRequest::get()
            .uri("/comments")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<comments::Comment> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 0);
    }

    #[actix_rt::test]
    async fn test_alert_resource() {
        setup();

        // Find all alerts, there should only be the initial one
        let mut app = test::init_service(AppFactory!()()).await;
        let req = test::TestRequest::get()
            .uri("/alerts")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<alerts::Alert> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 1);

        // Create an alert with ADMIN USER as the user_id
        let value = alerts::MaybeAlert {
            message: Some(String::from("foo")),
            reason: String::from("bar"),
            user_id: ADMIN_USER.id,
        };
        let payload = serde_json::to_string(&value).expect("Invalid value");

        let req = test::TestRequest::post()
            .uri("/alerts")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();
        let resp: alerts::Alert = test::read_response_json(&mut app, req).await;
        assert_eq!(value.message, resp.message);
        assert_eq!(value.reason, resp.reason);
        assert_eq!(value.user_id, resp.user_id);

        // Find all alerts, it should include the one we just created
        let req = test::TestRequest::get()
            .uri("/alerts")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<alerts::Alert> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 2);
        assert_eq!(value.message, resp[1].message);
        assert_eq!(value.reason, resp[1].reason);
        assert_eq!(value.user_id, resp[1].user_id);

        // Find alert by id
        let id = resp[1].id;

        let req = test::TestRequest::get()
            .uri(format!("/alerts/id/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: alerts::Alert = test::read_response_json(&mut app, req).await;
        assert_eq!(id, resp.id);
        assert_eq!(value.message, resp.message);
        assert_eq!(value.reason, resp.reason);
        assert_eq!(value.user_id, resp.user_id);

        // Update alert by id
        let value_updated = alerts::MaybeAlert {
            message: Some(String::from("foofoo")),
            reason: String::from("barbar"),
            user_id: ADMIN_USER.id,
        };
        let payload_updated = serde_json::to_string(&value_updated).expect("Invalid value");

        let req = test::TestRequest::put()
            .uri(format!("/alerts/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload_updated)
            .to_request();
        let resp: alerts::Alert = test::read_response_json(&mut app, req).await;
        assert_eq!(value_updated.message, resp.message);
        assert_eq!(value_updated.reason, resp.reason);
        assert_eq!(value_updated.user_id, resp.user_id);

        // Find alert by id, should be the updated one
        let req = test::TestRequest::get()
            .uri(format!("/alerts/id/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: alerts::Alert = test::read_response_json(&mut app, req).await;
        assert_eq!(id, resp.id);
        assert_eq!(value_updated.message, resp.message);
        assert_eq!(value_updated.reason, resp.reason);
        assert_eq!(value_updated.user_id, resp.user_id);

        // Delete the alert by id
        let req = test::TestRequest::delete()
            .uri(format!("/alerts/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: usize = test::read_response_json(&mut app, req).await;
        assert_eq!(1, resp);

        // Find all alerts, there should only be the initial one
        let req = test::TestRequest::get()
            .uri("/alerts")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<alerts::Alert> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 1);
    }

    #[actix_rt::test]
    async fn test_location_resource() {
        setup();

        // Find all locations, there should only be the initial one
        let mut app = test::init_service(AppFactory!()()).await;
        let req = test::TestRequest::get()
            .uri("/locations")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<locations::Location> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 1);

        // Create a location
        let value = locations::MaybeLocation {
            name: Some(String::from("foo")),
            latitude: f32::from(1.1),
            longitude: f32::from(2.2),
            ip: Some(IpNetwork::V4("10.9.0.32/16".parse().unwrap())),
        };

        let payload = serde_json::to_string(&value).expect("Invalid value");

        let req = test::TestRequest::post()
            .uri("/locations")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();
        let resp: locations::Location = test::read_response_json(&mut app, req).await;
        assert_eq!(value.name, resp.name);
        assert_eq!(value.latitude, resp.latitude);
        assert_eq!(value.longitude, resp.longitude);
        assert_eq!(value.ip, resp.ip);

        // Find all locations, it should include the one we just created
        let req = test::TestRequest::get()
            .uri("/locations")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<locations::Location> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 2);
        assert_eq!(value.name, resp[1].name);
        assert_eq!(value.latitude, resp[1].latitude);
        assert_eq!(value.longitude, resp[1].longitude);
        assert_eq!(value.ip, resp[1].ip);

        // Find location by id
        let id = resp[1].id;

        let req = test::TestRequest::get()
            .uri(format!("/locations/id/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: locations::Location = test::read_response_json(&mut app, req).await;
        assert_eq!(id, resp.id);
        assert_eq!(value.name, resp.name);
        assert_eq!(value.latitude, resp.latitude);
        assert_eq!(value.longitude, resp.longitude);
        assert_eq!(value.ip, resp.ip);

        // Update location by id
        let value_updated = locations::MaybeLocation {
            name: Some(String::from("foobar")),
            latitude: f32::from(10.10),
            longitude: f32::from(12.12),
            ip: Some(IpNetwork::V4("10.0.0.00/16".parse().unwrap())),
        };
        let payload_updated = serde_json::to_string(&value_updated).expect("Invalid value");

        let req = test::TestRequest::put()
            .uri(format!("/locations/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload_updated)
            .to_request();
        let resp: locations::Location = test::read_response_json(&mut app, req).await;
        assert_eq!(value_updated.name, resp.name);
        assert_eq!(value_updated.latitude, resp.latitude);
        assert_eq!(value_updated.longitude, resp.longitude);
        assert_eq!(value_updated.ip, resp.ip);

        // Find location by id, should be the updated one
        let req = test::TestRequest::get()
            .uri(format!("/locations/id/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: locations::Location = test::read_response_json(&mut app, req).await;
        assert_eq!(id, resp.id);
        assert_eq!(value_updated.name, resp.name);
        assert_eq!(value_updated.latitude, resp.latitude);
        assert_eq!(value_updated.longitude, resp.longitude);
        assert_eq!(value_updated.ip, resp.ip);

        // Delete the location by id
        let req = test::TestRequest::delete()
            .uri(format!("/locations/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: usize = test::read_response_json(&mut app, req).await;
        assert_eq!(1, resp);

        // Find all locations, there should only be the initial one now
        let req = test::TestRequest::get()
            .uri("/locations")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<locations::Location> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 1);
    }

    #[actix_rt::test]
    async fn test_room_resource() {
        setup();

        // Find all rooms, there should be none
        let mut app = test::init_service(AppFactory!()()).await;
        let req = test::TestRequest::get()
            .uri("/rooms")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<rooms::Room> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 0);

        // Create a room with INITIAL LOCATION as location association
        let value = rooms::MaybeRoom {
            name: String::from("foo"),
            location_id: INITIAL_LOCATION.id,
        };
        let payload = serde_json::to_string(&value).expect("Invalid value");

        let req = test::TestRequest::post()
            .uri("/rooms")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();
        let resp: rooms::Room = test::read_response_json(&mut app, req).await;
        assert_eq!(value.name, resp.name);
        assert_eq!(value.location_id, resp.location_id);

        // Find all rooms, it should be the one we just created
        let req = test::TestRequest::get()
            .uri("/rooms")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<rooms::Room> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 1);
        assert_eq!(value.name, resp[0].name);
        assert_eq!(value.location_id, resp[0].location_id);

        // Find room by id
        let id = resp[0].id;

        let req = test::TestRequest::get()
            .uri(format!("/rooms/id/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: rooms::Room = test::read_response_json(&mut app, req).await;
        assert_eq!(id, resp.id);
        assert_eq!(value.name, resp.name);
        assert_eq!(value.location_id, resp.location_id);

        // Update room by id
        let value_updated = rooms::MaybeRoom {
            name: String::from("foobar"),
            location_id: INITIAL_LOCATION.id,
        };
        let payload_updated = serde_json::to_string(&value_updated).expect("Invalid value");

        let req = test::TestRequest::put()
            .uri(format!("/rooms/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload_updated)
            .to_request();
        let resp: rooms::Room = test::read_response_json(&mut app, req).await;
        assert_eq!(value_updated.name, resp.name);
        assert_eq!(value_updated.location_id, resp.location_id);

        // Find room by id, should be the updated one
        let req = test::TestRequest::get()
            .uri(format!("/rooms/id/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: rooms::Room = test::read_response_json(&mut app, req).await;
        assert_eq!(id, resp.id);
        assert_eq!(value_updated.name, resp.name);
        assert_eq!(value_updated.location_id, resp.location_id);

        // Delete the room by id
        let req = test::TestRequest::delete()
            .uri(format!("/rooms/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: usize = test::read_response_json(&mut app, req).await;
        assert_eq!(1, resp);

        // Find all rooms, there should be none now
        let req = test::TestRequest::get()
            .uri("/rooms")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<rooms::Room> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 0);
    }

    #[actix_rt::test]
    async fn test_contact_event_resource() {
        setup();

        // Find all contact_events, there should be none
        let mut app = test::init_service(AppFactory!()()).await;
        let req = test::TestRequest::get()
            .uri("/contact_events")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<contact_events::ContactEvent> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 0);

        /* Create a contact_event with
            INITIAL ASSET TAG as asset tag association
            INITIAL LOCATION as location association
            INITIAL ALERT as alert association
        */
        let value = contact_events::MaybeContactEvent {
            asset_tag_id: INITIAL_ASSET_TAG.id,
            location_id: INITIAL_LOCATION.id,
            alert_id: Some(INITIAL_ALERT.id),
            deleted: false,
        };
        let payload = serde_json::to_string(&value).expect("Invalid value");

        let req = test::TestRequest::post()
            .uri("/contact_events")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .to_request();
        let resp: contact_events::ContactEvent = test::read_response_json(&mut app, req).await;
        assert_eq!(value.asset_tag_id, resp.asset_tag_id);
        assert_eq!(value.location_id, resp.location_id);
        assert_eq!(value.alert_id, resp.alert_id);
        assert_eq!(value.deleted, resp.deleted);

        // Find all contact_events, it should be the one we just created
        let req = test::TestRequest::get()
            .uri("/contact_events")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<contact_events::ContactEvent> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 1);
        assert_eq!(value.asset_tag_id, resp[0].asset_tag_id);
        assert_eq!(value.location_id, resp[0].location_id);
        assert_eq!(value.alert_id, resp[0].alert_id);
        assert_eq!(value.deleted, resp[0].deleted);

        // Find contact_event by id
        let id = resp[0].id;

        let req = test::TestRequest::get()
            .uri(format!("/contact_events/id/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: contact_events::ContactEvent = test::read_response_json(&mut app, req).await;
        assert_eq!(id, resp.id);
        assert_eq!(value.asset_tag_id, resp.asset_tag_id);
        assert_eq!(value.location_id, resp.location_id);
        assert_eq!(value.alert_id, resp.alert_id);
        assert_eq!(value.deleted, resp.deleted);

        // Update contact_event by id
        let value_updated = contact_events::MaybeContactEvent {
            asset_tag_id: INITIAL_ASSET_TAG.id,
            location_id: INITIAL_LOCATION.id,
            alert_id: Some(INITIAL_ALERT.id),
            deleted: false,
        };
        let payload_updated = serde_json::to_string(&value_updated).expect("Invalid value");

        let req = test::TestRequest::put()
            .uri(format!("/contact_events/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload_updated)
            .to_request();
        let resp: contact_events::ContactEvent = test::read_response_json(&mut app, req).await;
        assert_eq!(value_updated.asset_tag_id, resp.asset_tag_id);
        assert_eq!(value_updated.location_id, resp.location_id);
        assert_eq!(value_updated.alert_id, resp.alert_id);
        assert_eq!(value_updated.deleted, resp.deleted);

        // Find contact_event by id, should be the updated one
        let req = test::TestRequest::get()
            .uri(format!("/contact_events/id/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: contact_events::ContactEvent = test::read_response_json(&mut app, req).await;
        assert_eq!(value_updated.asset_tag_id, resp.asset_tag_id);
        assert_eq!(value_updated.location_id, resp.location_id);
        assert_eq!(value_updated.alert_id, resp.alert_id);
        assert_eq!(value_updated.deleted, resp.deleted);

        // Delete the contact_event by id
        let req = test::TestRequest::delete()
            .uri(format!("/contact_events/{}", id).as_str())
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: contact_events::ContactEvent = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.deleted, true);

        // Find all contact_events, there should be none
        let req = test::TestRequest::get()
            .uri("/contact_events")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<contact_events::ContactEvent> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 0);

        // Find all deleted contact_events, there should be the deleted one
        let req = test::TestRequest::get()
            .uri("/contact_events/deleted")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<contact_events::ContactEvent> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 1);
        assert_eq!(id, resp[0].id);
        assert_eq!(resp[0].deleted, true);

        // Find all contact_events including deleted ones, there should be 1
        let req = test::TestRequest::get()
            .uri("/contact_events/all")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<contact_events::ContactEvent> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 1);
    }
}
