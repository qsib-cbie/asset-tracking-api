use actix_web_httpauth::extractors::{AuthenticationError, bearer::{BearerAuth, Config}};
use actix_web::{Error, dev::ServiceRequest};

use super::users;

pub fn init() {
    users::init();
}

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    match credentials.token() {
        "_" => {
            if req.path() == "/health" {
                Ok(req)
            } else {
                let config = req.app_data::<Config>()
                    .map(|data| data.clone())
                    .unwrap_or_else(Default::default);

                Err(AuthenticationError::from(config).into())
            }
        },
        token => {
            match users::User::find_by_token(String::from(token)) {
                Ok(record) => {
                    log::trace!("Allowing user: {:?}", record);
                    Ok(req)
                },
                Err(_) => {
                    let config = req.app_data::<Config>()
                        .map(|data| data.clone())
                        .unwrap_or_else(Default::default);

                    Err(AuthenticationError::from(config).into())
                }
            }
        }
    }
}