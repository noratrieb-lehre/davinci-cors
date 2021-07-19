use actix_web::error::ErrorUnauthorized;
use actix_web::http::header::Header;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_httpauth::headers::authorization;
use actix_web_httpauth::headers::authorization::Bearer;

mod auth;
mod class;

pub type HttpResult = Result<HttpResponse, actix_web::Error>;

pub fn config(cfg: &mut web::ServiceConfig) {
    class::class_config(cfg);
    other_config(cfg);
}

pub fn user_config(cfg: &mut web::ServiceConfig) {}

pub fn other_config(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/hugo",
        web::get().to(|| HttpResponse::Ok().body("Hugo Boss")),
    )
    .route("/token", web::get().to(refresh_token));
}

async fn refresh_token(req: HttpRequest) -> HttpResult {
    let claims = match authorization::Authorization::<Bearer>::parse(&req) {
        Ok(auth) => auth::validate_token(auth.into_scheme().token()),
        Err(_) => Err(ErrorUnauthorized("No Bearer token present")),
    }?;

    if claims.refresh {
        let new_token = auth::create_normal_jwt(claims.uid)?;
        Ok(HttpResponse::Ok()
            .header("Token", new_token.0)
            .json(dao::RefreshResponse {
                expires: new_token.1,
            }))
    } else {
        Err(ErrorUnauthorized(
            "Normal token cannot be used to get a new token",
        ))
    }
}
