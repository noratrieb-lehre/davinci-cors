use actix_web::{HttpResponse, ResponseError};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ServiceErr {
    InternalServerError,
    ConnectionNotFound(r2d2::Error),
    DbActionFailed(diesel::result::Error),
    JWTCreationError(jsonwebtoken::errors::Error),
    TokenExpiredError,
    JWTokenError,
}

impl std::error::Error for ServiceErr {}

impl Display for ServiceErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ServiceErr::ConnectionNotFound(err) => format!("Db Connection not found: {}", err),
                ServiceErr::DbActionFailed(err) => format!("Db Action Failed: {}", err),
                ServiceErr::JWTCreationError(err) => format!("Could not create JWT: {}", err),
                ServiceErr::InternalServerError => format!("Internal Server error"),
                ServiceErr::TokenExpiredError => format!("Token expired."),
                ServiceErr::JWTokenError => format!("Invalid JWT"),
            }
        )
    }
}

impl ResponseError for ServiceErr {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceErr::TokenExpiredError => HttpResponse::Unauthorized().body("Token expired."),
            ServiceErr::JWTokenError => HttpResponse::BadRequest().body("Token expired."),
            err => HttpResponse::InternalServerError().body(format!("{}", err)),
        }
    }
}

impl From<diesel::result::Error> for ServiceErr {
    fn from(err: diesel::result::Error) -> Self {
        Self::DbActionFailed(err)
    }
}

impl From<r2d2::Error> for ServiceErr {
    fn from(err: r2d2::Error) -> Self {
        Self::ConnectionNotFound(err)
    }
}
