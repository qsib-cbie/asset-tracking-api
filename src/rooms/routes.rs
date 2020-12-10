use crate::error_handler::CustomError;
use crate::rooms::{MaybeRoom, Room};
use actix_web::{delete, get, post, put, web, HttpResponse};
use log;

#[get("/rooms")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let rooms = Room::find_all()?;
    Ok(HttpResponse::Ok().json(rooms))
}

#[get("/rooms/id/{id}")]
async fn find_by_id(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("GET /rooms/id/{}", &id);
    let room = Room::find_by_id(id)?;
    Ok(HttpResponse::Ok().json(room))
}

#[get("/rooms/name/{name}")]
async fn find_by_name(name: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let name = name.into_inner();
    log::trace!("GET /rooms/name/{}", &name);
    let room = Room::find_by_name(name)?;
    Ok(HttpResponse::Ok().json(room))
}

#[get("/rooms/location/{id}")]
async fn find_by_location(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("GET /rooms/location/{}", &id);
    let rooms = Room::find_by_location(id)?;
    Ok(HttpResponse::Ok().json(rooms))
}

#[post("/rooms")]
async fn create(room: web::Json<MaybeRoom>) -> Result<HttpResponse, CustomError> {
    let room = room.into_inner();
    log::trace!("POST /rooms/ {:?}", &room);
    let room = Room::create(room)?;
    Ok(HttpResponse::Ok().json(room))
}

#[put("/rooms/{id}")]
async fn update(
    id: web::Path<i64>,
    room: web::Json<MaybeRoom>,
) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    let room = room.into_inner();
    log::trace!("PUT /rooms/{} {:?}", &id, &room);
    let room = Room::update(id, room)?;
    Ok(HttpResponse::Ok().json(room))
}

#[delete("/rooms/{id}")]
async fn delete(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("DELETE /rooms/{}", &id);
    let res = Room::delete(id)?;
    Ok(HttpResponse::Ok().json(res))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find_by_id);
    comfig.service(find_by_name);
    comfig.service(find_by_location);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}
