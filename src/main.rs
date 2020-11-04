#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_service::Service;
use actix_web::{App, Error, HttpServer, dev::ServiceRequest};
use actix_web::middleware::Logger;
use actix_web_httpauth::extractors::{AuthenticationError, bearer::{BearerAuth, Config}};
use actix_web_httpauth::middleware::HttpAuthentication;

use http::header;

use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod db;
mod health;
mod error_handler;
mod schema;


async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    match credentials.token() {
        "none" => {
            if req.path() == "/health" {
                Ok(req)
            } else {
                let config = req.app_data::<Config>()
                    .map(|data| data.clone())
                    .unwrap_or_else(Default::default);

                Err(AuthenticationError::from(config).into())
            }
        }
        _ => {
                let config = req.app_data::<Config>()
                    .map(|data| data.clone())
                    .unwrap_or_else(Default::default);

                Err(AuthenticationError::from(config).into())
        }
    }
}

macro_rules! AppFactory {
    () => {
        || App::new()
        .wrap(Logger::default())
        .wrap(HttpAuthentication::bearer(validator))
        .wrap_fn(|req, srv| {
            let mut req: ServiceRequest = req.into();
            let headers = req.headers_mut();
            if !headers.contains_key("authorization") {
                headers.insert(header::HeaderName::from_static("authorization"), header::HeaderValue::from_static("Bearer none"))
            }

            srv.call(req)
        })
        .configure(health::init_routes)
    };
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    db::init();

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
    use actix_web::test;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    struct Empty { }

    #[actix_rt::test]
    async fn test_health_get_without_token() {
        let mut app = test::init_service(AppFactory!()()).await;
        let req = test::TestRequest::get()
            .uri("/health")
            .to_request();
        let _resp = test::read_response(&mut app, req).await;
    }
}