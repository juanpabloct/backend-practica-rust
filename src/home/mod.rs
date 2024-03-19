use crate::home::utils::home_utils::{read, read_dynamic, stream};
use actix_web::web;
mod utils;

pub fn home(cfg: &mut web::ServiceConfig) {
    // let app_state = Name2 {};

    cfg.service(read);
    cfg.service(read_dynamic);
    cfg.service(stream);
}
