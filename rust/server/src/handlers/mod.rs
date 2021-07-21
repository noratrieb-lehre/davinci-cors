use crate::error::ServiceErr;
use actix_web::http::header::Header;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_httpauth::headers::authorization;
use actix_web_httpauth::headers::authorization::Bearer;

mod auth;
mod class;

macro_rules! http_todo {
    () => {
        std::result::Result::Ok(actix_web::HttpResponse::Ok().body("Unimplemented"))
    };
    ($str:literal) => {
        std::result::Result::Ok(actix_web::HttpResponse::Ok().body($str))
    };
}

pub type HttpResult = Result<HttpResponse, ServiceErr>;

pub fn config(cfg: &mut web::ServiceConfig) {
    other_config(cfg);
    class::class_config(cfg);
}

pub fn other_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/hugo", web::get().to(get_hugo))
        .route("/token", web::get().to(refresh_token))
        .route("/login", web::post().to(login))
        .service(
            web::scope("/users/me")
                .route("", web::get().to(get_own_user))
                .route("", web::put().to(edit_own_user)),
        );
}

async fn get_hugo() -> HttpResponse {
    HttpResponse::Ok().body("Hugo Boss")
}

async fn refresh_token(req: HttpRequest) -> HttpResult {
    let claims = match authorization::Authorization::<Bearer>::parse(&req) {
        Ok(auth) => auth::validate_token(auth.into_scheme().token()),
        Err(_) => Err(ServiceErr::Unauthorized("No Bearer token present")),
    }?;

    if claims.refresh {
        let new_token = auth::create_normal_jwt(claims.uid)?;
        Ok(HttpResponse::Ok()
            .header("Token", new_token.0)
            .json(dao::RefreshResponse {
                expires: new_token.1,
            }))
    } else {
        Err(ServiceErr::Unauthorized(
            "Normal token cannot be used to get a new token",
        ))
    }
}

async fn login() -> HttpResult {
    http_todo!()
}

async fn get_own_user() -> HttpResult {
    http_todo!()
}

async fn edit_own_user() -> HttpResult {
    http_todo!()
}
