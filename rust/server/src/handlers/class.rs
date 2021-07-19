use crate::handlers::HttpResult;
use actix_web::web;

pub(super) fn class_config(cfg: &mut web::ServiceConfig) {}

fn get_class() -> HttpResult {
    todo!()
}
