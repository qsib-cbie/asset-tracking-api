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

mod asset_scanners;
mod asset_tags;
mod health;
mod roles;
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
                .configure(asset_tags::init_routes)
                .configure(asset_scanners::init_routes)                
                .configure(health::init_routes)                
                .configure(roles::init_routes)
                .configure(users::init_routes)
                
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
    use actix_web::{test, App, http::StatusCode};
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
    }

    pub fn setup() {
        lazy_static::initialize(&FIXTURE);
        lazy_static::initialize(&ADMIN_USER);
    }

    #[derive(Serialize, Deserialize)]
    struct Empty {}

    #[actix_rt::test]
    async fn test_health_get_without_token() {
        setup();

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
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", user1.token),
            )
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
        let _protected_resp: Vec<asset_tags::AssetTag> = test::read_response_json(&mut app, req).await;

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
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", user2.token),
            )
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
        let _protected_resp: Vec<asset_tags::AssetTag> = test::read_response_json(&mut app, req).await;

        // Use user2's token
        let req = test::TestRequest::get()
            .uri("/asset_tags")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", user2.token).as_str(),
            )
            .to_request();
        let _protected_resp: Vec<asset_tags::AssetTag> = test::read_response_json(&mut app, req).await;
    }

    #[actix_rt::test]
    async fn test_asset_tags_resource() {
        setup();

        // Find all tags, there should be none
        let mut app = test::init_service(AppFactory!()()).await;
        let req = test::TestRequest::get()
            .uri("/asset_tags")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<asset_tags::AssetTag> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 0);

        // Create a tag
        let value = asset_tags::MaybeAssetTag {
            name: String::from("foo"),
            description: Some(String::from("bar")),
            serial_number: String::from("asdf"),
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

        // Find all tags, it should be the one we just created
        let req = test::TestRequest::get()
            .uri("/asset_tags")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<asset_tags::AssetTag> = test::read_response_json(&mut app, req).await;
        assert_eq!(resp.len(), 1);
        assert_eq!(value.name, resp[0].name);
        assert_eq!(value.description, resp[0].description);
        assert_eq!(value.serial_number, resp[0].serial_number);

        // Create another tag
        let another_value = asset_tags::MaybeAssetTag {
            name: String::from("foo1"),
            description: Some(String::from("asdflkj")),
            serial_number: String::from("asdf1"),
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

        // Find all tags, it should be the two we just created
        let req = test::TestRequest::get()
            .uri("/asset_tags")
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", ADMIN_USER.token),
            )
            .to_request();
        let resp: Vec<asset_tags::AssetTag> = test::read_response_json(&mut app, req).await;

        // This order is not guaranteed by the endpoint. It is an undefined side effect of the underlying postgres query.
        assert_eq!(resp.len(), 2);
        assert_eq!(value.name, resp[0].name);
        assert_eq!(value.description, resp[0].description);
        assert_eq!(value.serial_number, resp[0].serial_number);
        assert_eq!(another_value.name, resp[1].name);
        assert_eq!(another_value.description, resp[1].description);
        assert_eq!(another_value.serial_number, resp[1].serial_number);
    }
}
