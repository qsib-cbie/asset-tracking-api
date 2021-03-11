use crate::contact_events::{ContactEvent, MaybeContactEvent};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use log;

#[get("/contact_events")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let contact_events = ContactEvent::find_all()?;
    Ok(HttpResponse::Ok().json(contact_events))
}

#[get("/contact_events/all")]
async fn find_with_deleted() -> Result<HttpResponse, CustomError> {
    let contact_events = ContactEvent::find_with_deleted()?;
    Ok(HttpResponse::Ok().json(contact_events))
}

#[get("/contact_events/deleted")]
async fn find_deleted() -> Result<HttpResponse, CustomError> {
    let contact_events = ContactEvent::find_deleted()?;
    Ok(HttpResponse::Ok().json(contact_events))
}

#[get("/contact_events/id/{id}")]
async fn find_by_id(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("GET /contact_events/id/{}", &id);
    let contact_event = ContactEvent::find_by_id(id)?;
    Ok(HttpResponse::Ok().json(contact_event))
}

#[get("/contact_events/asset_tag/{id}")]
async fn find_by_asset_tag(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("GET /contact_events/asset_tag/{}", &id);
    let contact_events = ContactEvent::find_by_asset_tag(id)?;
    Ok(HttpResponse::Ok().json(contact_events))
}

#[get("/contact_events/location/{id}")]
async fn find_by_location(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("GET /contact_events/location/{}", &id);
    let contact_events = ContactEvent::find_by_location(id)?;
    Ok(HttpResponse::Ok().json(contact_events))
}

#[get("/contact_events/alert/{id}")]
async fn find_by_alert(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("GET /contact_events/alert/{}", &id);
    let contact_events = ContactEvent::find_by_alert(id)?;
    Ok(HttpResponse::Ok().json(contact_events))
}

#[post("/contact_events")]
async fn create(contact_event: web::Json<MaybeContactEvent>) -> Result<HttpResponse, CustomError> {
    let contact_event = contact_event.into_inner();
    log::trace!("POST /contact_events/ {:?}", &contact_event);
    let contact_event = ContactEvent::create(contact_event)?;
    Ok(HttpResponse::Ok().json(contact_event))
}

#[put("/contact_events/{id}")]
async fn update(
    id: web::Path<i64>,
    contact_event: web::Json<MaybeContactEvent>,
) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    let contact_event = contact_event.into_inner();
    log::trace!("PUT /contact_events/{} {:?}", &id, &contact_event);
    let contact_event = ContactEvent::update(id, contact_event)?;
    Ok(HttpResponse::Ok().json(contact_event))
}

#[delete("/contact_events/{id}")]
async fn delete(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("DELETE /contact_events/{}", &id);
    let res = ContactEvent::delete(id)?;
    Ok(HttpResponse::Ok().json(res))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find_with_deleted);
    comfig.service(find_deleted);
    comfig.service(find_by_id);
    comfig.service(find_by_asset_tag);
    comfig.service(find_by_location);
    comfig.service(find_by_alert);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}
