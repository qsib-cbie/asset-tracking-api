use crate::error_handler::CustomError;
use crate::locations::{Location, MaybeLocation};
use actix_web::{delete, get, post, put, web, HttpResponse};
use ipnetwork::IpNetwork;
use log;

#[get("/locations")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let locations = Location::find_all()?;
    Ok(HttpResponse::Ok().json(locations))
}

#[get("/locations/id/{id}")]
async fn find_by_id(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("GET /locations/id/{}", &id);
    let location = Location::find_by_id(id)?;
    Ok(HttpResponse::Ok().json(location))
}

#[get("/locations/name/{name}")]
async fn find_by_name(name: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let name = name.into_inner();
    log::trace!("GET /locations/name/{}", &name);
    let location = Location::find_by_name(name)?;
    Ok(HttpResponse::Ok().json(location))
}

#[get("/locations/ip/{ip}")]
async fn find_by_ip(ip: web::Path<IpNetwork>) -> Result<HttpResponse, CustomError> {
    let ip = ip.into_inner();
    log::trace!("GET /locations/ip/{}", &ip);
    let location = Location::find_by_ip(ip)?;
    Ok(HttpResponse::Ok().json(location))
}

#[post("/locations")]
async fn create(location: web::Json<MaybeLocation>) -> Result<HttpResponse, CustomError> {
    let location = location.into_inner();
    log::trace!("POST /locations/ {:?}", &location);
    let location = Location::create(location)?;
    Ok(HttpResponse::Ok().json(location))
}

#[put("/locations/{id}")]
async fn update(
    id: web::Path<i64>,
    location: web::Json<MaybeLocation>,
) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    let location = location.into_inner();
    log::trace!("PUT /locations/{} {:?}", &id, &location);
    let location = Location::update(id, location)?;
    Ok(HttpResponse::Ok().json(location))
}

#[delete("/locations/{id}")]
async fn delete(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("DELETE /locations/{}", &id);
    let res = Location::delete(id)?;
    Ok(HttpResponse::Ok().json(res))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find_by_id);
    comfig.service(find_by_name);
    comfig.service(find_by_ip);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}
