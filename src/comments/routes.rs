use crate::comments::{Comment, MaybeComment};
use crate::error_handler::CustomError;
use actix_web::{get, post, put, delete, web, HttpResponse};
use log;

#[get("/comments")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let comments = Comment::find_all()?;
    Ok(HttpResponse::Ok().json(comments))
}

#[get("/comments/id/{id}")]
async fn find_by_id(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("GET /comments/id/{}", &id);
    let comment = Comment::find_by_id(id)?;
    Ok(HttpResponse::Ok().json(comment))
}

#[get("/comments/user/{id}")]
async fn find_by_user(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("GET /comments/user/{}", &id);
    let comments = Comment::find_by_user(id)?;
    Ok(HttpResponse::Ok().json(comments))
}

#[get("/comments/asset_tag/{id}")]
async fn find_by_asset_tag(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("GET /comments/asset_tag/{}", &id);
    let comments = Comment::find_by_asset_tag(id)?;
    Ok(HttpResponse::Ok().json(comments))
}

#[post("/comments")]
async fn create(comment: web::Json<MaybeComment>) -> Result<HttpResponse, CustomError> {
    let comment = comment.into_inner();
    log::trace!("POST /comments/ {:?}", &comment);
    let comment = Comment::create(comment)?;
    Ok(HttpResponse::Ok().json(comment))
}

#[put("/comments/{id}")]
async fn update(
    id: web::Path<i64>,
    comment: web::Json<MaybeComment>,
) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    let comment = comment.into_inner();
    log::trace!("PUT /comments/{} {:?}", &id, &comment);
    let comment = Comment::update(id, comment)?;
    Ok(HttpResponse::Ok().json(comment))
}

#[delete("/comments/{id}")]
async fn delete(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("DELETE /comments/{}", &id);
    let res = Comment::delete(id)?;
    Ok(HttpResponse::Ok().json(res))    
}



pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find_by_id);    
    comfig.service(find_by_user);
    comfig.service(find_by_asset_tag);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete); 
}
