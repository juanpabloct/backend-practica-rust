mod utils;
use actix_web::{web, HttpResponse};
use utils::read::read_buy;
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(read_buy))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
