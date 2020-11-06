use crate::users::{AuthUser, User, MaybeUser};
use crate::error_handler::CustomError;
use actix_web::{get, post, web, HttpResponse};
use log;
use std::convert::TryInto;

#[get("/users/token/{token}")]
async fn find_by_token(token: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let token = token.into_inner();
    log::trace!("GET /users/token/{}", &token);
    let user = User::find_by_token(token)?;
    let auth_user: AuthUser = user.try_into()?;
    Ok(HttpResponse::Ok().json(auth_user))
}

#[post("/users")]
async fn create(user: web::Json<MaybeUser>) -> Result<HttpResponse, CustomError> {
    let user = user.into_inner();
    log::trace!("POST /users");
    let user = User::create(user)?;
    let auth_user: AuthUser = user.try_into()?;
    Ok(HttpResponse::Ok().json(auth_user))
}


pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_by_token);
    comfig.service(create);
}
