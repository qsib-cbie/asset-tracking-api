use crate::error_handler::CustomError;
use actix_web::{get, web, HttpResponse};

#[get("/health")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    Ok(HttpResponse::Ok().json({}))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
}
