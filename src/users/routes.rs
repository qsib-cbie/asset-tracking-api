use crate::users::{AuthUser, User, MaybeUser};
use crate::error_handler::CustomError;
use actix_web::{post, put, web, HttpResponse};
use log;
use std::convert::TryInto;

#[put("/users/{id}")]
async fn update(id: web::Path<i64>, user: web::Json<MaybeUser>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    let user = user.into_inner();
    log::trace!("PUT /users/{}", id);
    let user = User::update(id, user)?;
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
    comfig.service(update);
    comfig.service(create);
}
