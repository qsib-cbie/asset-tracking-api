use crate::error_handler::CustomError;
use crate::roles::{MaybeRole, Role};
use actix_web::{delete, get, post, put, web, HttpResponse};
use log;

#[get("/roles")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let roles = Role::find_all()?;
    Ok(HttpResponse::Ok().json(roles))
}

#[get("/roles/id/{id}")]
async fn find_by_id(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("GET /roles/id/{}", &id);
    let role = Role::find_by_id(id)?;
    Ok(HttpResponse::Ok().json(role))
}

#[get("/roles/name/{name}")]
async fn find_by_name(name: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let name = name.into_inner();
    log::trace!("GET /roles/name/{}", &name);
    let role = Role::find_by_name(name)?;
    Ok(HttpResponse::Ok().json(role))
}

#[get("/roles/user/{id}")]
async fn find_by_user(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("GET /roles/user/{}", &id);
    let roles = Role::find_by_user(id)?;
    Ok(HttpResponse::Ok().json(roles))
}

#[post("/roles")]
async fn create(role: web::Json<MaybeRole>) -> Result<HttpResponse, CustomError> {
    let role = role.into_inner();
    log::trace!("POST /roles/ {:?}", &role);
    let role = Role::create(role)?;
    Ok(HttpResponse::Ok().json(role))
}

#[put("/roles/{id}")]
async fn update(
    id: web::Path<i64>,
    role: web::Json<MaybeRole>,
) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    let role = role.into_inner();
    log::trace!("PUT /roles/{} {:?}", &id, &role);
    let role = Role::update(id, role)?;
    Ok(HttpResponse::Ok().json(role))
}

#[delete("/roles/{id}")]
async fn delete(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("DELETE /roles/{}", &id);
    let res = Role::delete(id)?;
    Ok(HttpResponse::Ok().json(res))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find_by_id);
    comfig.service(find_by_name);
    comfig.service(find_by_user);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}
