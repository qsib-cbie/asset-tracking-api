use crate::asset_scanners::{AssetScanner, MaybeAssetScanner};
use crate::error_handler::CustomError;
use actix_web::{get, post, put, delete, web, HttpResponse};
use log;


#[get("/asset_scanners")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let asset_scanners = AssetScanner::find_all()?;
    Ok(HttpResponse::Ok().json(asset_scanners))
}

#[get("/asset_scanners/id/{id}")]
async fn find_by_id(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("GET /asset_scanners/id/{}", &id);
    let asset_scanner = AssetScanner::find_by_id(id)?;
    Ok(HttpResponse::Ok().json(asset_scanner))
}

#[get("/asset_scanners/name/{name}")]
async fn find_by_name(name: web::Path<String>) -> Result<HttpResponse, CustomError> {
    let name = name.into_inner();
    log::trace!("GET /asset_scanners/name/{}", &name);
    let asset_scanner = AssetScanner::find_by_name(name)?;
    Ok(HttpResponse::Ok().json(asset_scanner))
}

#[post("/asset_scanners")]
async fn create(asset_scanner: web::Json<MaybeAssetScanner>) -> Result<HttpResponse, CustomError> {
    let asset_scanner = asset_scanner.into_inner();
    log::trace!("POST /asset_scanners/ {:?}", &asset_scanner);
    let asset_scanner = AssetScanner::create(asset_scanner)?;
    Ok(HttpResponse::Ok().json(asset_scanner))
}

#[put("/asset_scanners/{id}")]
async fn update(
    id: web::Path<i64>,
    asset_scanner: web::Json<MaybeAssetScanner>,
) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    let asset_scanner = asset_scanner.into_inner();
    log::trace!("PUT /asset_scanners/{} {:?}", &id, &asset_scanner);
    let asset_scanner = AssetScanner::update(id, asset_scanner)?;
    Ok(HttpResponse::Ok().json(asset_scanner))
}

#[delete("/asset_scanners/{id}")]
async fn delete(id: web::Path<i64>) -> Result<HttpResponse, CustomError> {
    let id = id.into_inner();
    log::trace!("DELETE /asset_scanners/{}", &id);
    let res = AssetScanner::delete(id)?;
    Ok(HttpResponse::Ok().json(res))    
}
    


pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);    
    comfig.service(find_by_id);
    comfig.service(find_by_name);
    comfig.service(create);
    comfig.service(update); 
    comfig.service(delete);   
}
