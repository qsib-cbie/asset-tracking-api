use crate::assets::{Asset, MaybeAsset};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use log;

#[get("/assets")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let assets = Asset::find_all()?;
    Ok(HttpResponse::Ok().json(assets))
}

#[get("/assets/all")]
async fn find_with_deleted() -> Result<HttpResponse, CustomError> {
    let assets = Asset::find_with_deleted()?;
    Ok(HttpResponse::Ok().json(assets))
}

#[get("/assets/deleted")]
async fn find_deleted() -> Result<HttpResponse, CustomError> {
    let assets = Asset::find_deleted()?;
    Ok(HttpResponse::Ok().json(assets))
}

#[get("/assets/id/{id}")]
async fn find_by_id(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("GET /assets/id/{}", &id);
    let asset = Asset::find_by_id(id)?;
    Ok(HttpResponse::Ok().json(asset))
}

#[get("/assets/asset_tag/{id}")]
async fn find_by_asset_tag(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("GET /assets/asset_tag/{}", &id);
    let assets = Asset::find_by_asset_tag(id)?;
    Ok(HttpResponse::Ok().json(assets))
}

#[post("/assets")]
async fn create(asset: web::Json<MaybeAsset>) -> Result<HttpResponse, CustomError> {
    let asset = asset.into_inner();
    log::trace!("POST /assets/ {:?}", &asset);
    let asset = Asset::create(asset)?;
    Ok(HttpResponse::Ok().json(asset))
}

#[put("/assets/{id}")]
async fn update(
    id: web::Path<i64>,
    asset: web::Json<MaybeAsset>,
) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    let asset = asset.into_inner();
    log::trace!("PUT /assets/{} {:?}", &id, &asset);
    let asset = Asset::update(id, asset)?;
    Ok(HttpResponse::Ok().json(asset))
}

#[delete("/assets/{id}")]
async fn delete(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("DELETE /assets/{}", &id);
    let res = Asset::delete(id)?;
    Ok(HttpResponse::Ok().json(res))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find_with_deleted);
    comfig.service(find_deleted);
    comfig.service(find_by_id);
    comfig.service(find_by_asset_tag);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}
