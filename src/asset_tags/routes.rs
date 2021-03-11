use crate::asset_tags::{AssetTag, MaybeAssetTag};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use log;

#[get("/asset_tags")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let asset_tags = AssetTag::find_all()?;
    Ok(HttpResponse::Ok().json(asset_tags))
}

#[get("/asset_tags/all")]
async fn find_with_deleted() -> Result<HttpResponse, CustomError> {
    let asset_tags = AssetTag::find_with_deleted()?;
    Ok(HttpResponse::Ok().json(asset_tags))
}

#[get("/asset_tags/deleted")]
async fn find_deleted() -> Result<HttpResponse, CustomError> {
    let asset_tags = AssetTag::find_deleted()?;
    Ok(HttpResponse::Ok().json(asset_tags))
}

#[get("/asset_tags/id/{id}")]
async fn find_by_id(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("GET /asset_tags/id/{}", &id);
    let asset_tag = AssetTag::find_by_id(id)?;
    Ok(HttpResponse::Ok().json(asset_tag))
}

#[get("/asset_tags/name/{name}")]
async fn find_by_name(name: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let name = name.into_inner();
    log::trace!("GET /asset_tags/name/{}", &name);
    let asset_tag = AssetTag::find_by_name(name)?;
    Ok(HttpResponse::Ok().json(asset_tag))
}

#[get("/asset_tags/asset_id/{id}")]
async fn find_by_asset(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("GET /asset_tags/asset_id/{}", &id);
    let asset_tags = AssetTag::find_by_asset(id)?;
    Ok(HttpResponse::Ok().json(asset_tags))
}

#[post("/asset_tags")]
async fn create(asset_tag: web::Json<MaybeAssetTag>) -> Result<HttpResponse, CustomError> {
    let asset_tag = asset_tag.into_inner();
    log::trace!("POST /asset_tags/ {:?}", &asset_tag);
    let asset_tag = AssetTag::create(asset_tag)?;
    Ok(HttpResponse::Ok().json(asset_tag))
}

#[put("/asset_tags/{id}")]
async fn update(
    id: web::Path<i64>,
    asset_tag: web::Json<MaybeAssetTag>,
) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    let asset_tag = asset_tag.into_inner();
    log::trace!("PUT /asset_tags/{} {:?}", &id, &asset_tag);
    let asset_tag = AssetTag::update(id, asset_tag)?;
    Ok(HttpResponse::Ok().json(asset_tag))
}

#[delete("/asset_tags/{id}")]
async fn delete(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("DELETE /asset_tags/{}", &id);
    let res = AssetTag::delete(id)?;
    Ok(HttpResponse::Ok().json(res))
}

#[delete("/asset_tags/asset_id/{id}")]
async fn delete_by_asset(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("DELETE /asset_tags/asset_id/{}", &id);
    let res = AssetTag::delete_by_asset(id)?;
    Ok(HttpResponse::Ok().json(res))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find_with_deleted);
    comfig.service(find_deleted);
    comfig.service(find_by_id);
    comfig.service(find_by_asset);
    comfig.service(find_by_name);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
    comfig.service(delete_by_asset);
}
