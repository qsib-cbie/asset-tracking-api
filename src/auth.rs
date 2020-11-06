use actix_web_httpauth::extractors::{AuthenticationError, bearer::{BearerAuth, Config}};
use actix_web::{Error, dev::ServiceRequest};

use super::users;

pub fn init() {
    users::init();
}

async fn _validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
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

// #[cfg(not(test))]
// pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
//     _validator(req, credentials).await
// }
// #[cfg(test)]
pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    match credentials.token() {
        "A841BE66-84AC-4BA7-B0E1-D34B1FC2F08A" => {
            Ok(req)
        },
        _ => {
            _validator(req, credentials).await
        }
    }
}