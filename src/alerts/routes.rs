use crate::alerts::{Alert, MaybeAlert};
use crate::error_handler::CustomError;
use actix_web::{get, post, put, delete, web, HttpResponse};
use log;

#[get("/alerts")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let alerts = Alert::find_all()?;
    Ok(HttpResponse::Ok().json(alerts))
}

#[get("/alerts/id/{id}")]
async fn find_by_id(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("GET /alerts/id/{}", &id);
    let alert = Alert::find_by_id(id)?;
    Ok(HttpResponse::Ok().json(alert))
}

#[get("/alerts/user/{id}")]
async fn find_by_user(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("GET /alerts/user/{}", &id);
    let alerts = Alert::find_by_user(id)?;
    Ok(HttpResponse::Ok().json(alerts))
}

#[post("/alerts")]
async fn create(alert: web::Json<MaybeAlert>) -> Result<HttpResponse, CustomError> {
    let alert = alert.into_inner();
    log::trace!("POST /alerts/ {:?}", &alert);
    let alert = Alert::create(alert)?;
    Ok(HttpResponse::Ok().json(alert))
}

#[put("/alerts/{id}")]
async fn update(
    id: web::Path<i64>,
    alert: web::Json<MaybeAlert>,
) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    let alert = alert.into_inner();
    log::trace!("PUT /alerts/{} {:?}", &id, &alert);
    let alert = Alert::update(id, alert)?;
    Ok(HttpResponse::Ok().json(alert))
}

#[delete("/alerts/{id}")]
async fn delete(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("DELETE /alerts/{}", &id);
    let res = Alert::delete(id)?;
    Ok(HttpResponse::Ok().json(res))    
}



pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find_by_id);    
    comfig.service(find_by_user);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete); 
}
