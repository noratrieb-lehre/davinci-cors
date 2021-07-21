use crate::error::ServiceErr;
use actix_web::{web, HttpResponse};
mod auth;
mod class;

macro_rules! http_todo {
    () => {
        std::result::Result::Ok(actix_web::HttpResponse::NotFound().body("Unimplemented"))
    };
}

pub type HttpResult = Result<HttpResponse, ServiceErr>;

pub fn config(cfg: &mut web::ServiceConfig) {
    other_config(cfg);
    class::class_config(cfg);
    auth::auth_config(cfg);
}

pub fn other_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/hugo", web::get().to(get_hugo)).service(
        web::scope("/users")
            .route("", web::post().to(create_user))
            .route("/me", web::get().to(get_own_user))
            .route("/me", web::put().to(edit_own_user))
            .route("/me", web::delete().to(delete_own_user)),
    );
}

async fn get_hugo() -> HttpResponse {
    HttpResponse::Ok().body("Hugo Boss")
}

async fn create_user() -> HttpResult {
    http_todo!()
}

async fn get_own_user() -> HttpResult {
    http_todo!()
}

async fn edit_own_user() -> HttpResult {
    http_todo!()
}

async fn delete_own_user() -> HttpResult {
    http_todo!()
}
