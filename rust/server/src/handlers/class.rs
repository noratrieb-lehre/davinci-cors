use crate::actions::{self, Pool};
use crate::handlers::auth::Claims;
use crate::handlers::HttpResult;
use actix_web::http::header::http_percent_encode;
use actix_web::web;
use std::str::FromStr;

pub(super) fn class_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/classes", web::post().to(create_class)).service(
        web::scope("/classes/{uuid}")
            .route("", web::get().to(get_class))
            .route("", web::put().to(edit_class))
            .route("", web::delete().to(delete_class))
            .route("/members/{uuid}", web::put().to(edit_member))
            .route("/join", web::post().to(request_join))
            .route("/requests", web::get().to(get_join_requests))
            .route("/requests/{uuid}", web::post().to(accept_member))
            .route("/events", web::get().to(get_events))
            .route("/events", web::post().to(create_event))
            .route("/events/{uuid}", web::get().to(get_event))
            .route("/events/{uuid}", web::put().to(edit_event))
            .route("/events/{uuid}", web::delete().to(delete_event))
            .route("timetable", web::get().to(get_timetable))
            .route("timetable", web::put().to(edit_timetable)),
    );
}

async fn get_class(params: web::Path<String>, db: web::Data<Pool>, claims: Claims) -> HttpResult {
    let uuid = uuid::Uuid::from_str(&params)?;

    let class = web::block(move || actions::class::get_class(&db, uuid)).await??;

    http_todo!()
}

async fn create_class() -> HttpResult {
    http_todo!()
}

async fn edit_class() -> HttpResult {
    http_todo!()
}

async fn delete_class() -> HttpResult {
    http_todo!()
}

async fn edit_member() -> HttpResult {
    http_todo!()
}

async fn request_join() -> HttpResult {
    http_todo!()
}

async fn get_join_requests() -> HttpResult {
    http_todo!()
}
async fn accept_member() -> HttpResult {
    http_todo!()
}

async fn get_event() -> HttpResult {
    http_todo!()
}

async fn get_events() -> HttpResult {
    http_todo!()
}

async fn create_event() -> HttpResult {
    http_todo!()
}

async fn edit_event() -> HttpResult {
    http_todo!()
}

async fn delete_event() -> HttpResult {
    http_todo!()
}

async fn get_timetable() -> HttpResult {
    http_todo!()
}

async fn edit_timetable() -> HttpResult {
    http_todo!()
}
