use actix_web::{web, HttpResponse};

mod class;

pub type HttpResult = Result<HttpResponse, actix_web::Error>;

pub fn class_config(cfg: &mut web::ServiceConfig) {

}

pub fn user_config(cfg: &mut web::ServiceConfig) {

}

pub fn other_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/hugo", web::get().to(|| HttpResponse::Ok().body("Hugo Boss")));
}