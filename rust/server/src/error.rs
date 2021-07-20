use actix_web::{HttpResponse, ResponseError};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::option::NoneError;

#[derive(Debug)]
pub enum ServiceErr {
    ConnectionNotFound(r2d2::Error),
    DbActionFailed(diesel::result::Error),
    NotFound,
    JWTCreationError(jsonwebtoken::errors::Error),
    TokenExpiredError,
    JWTokenError,
    ActixError(actix_web::Error),
}

impl std::error::Error for ServiceErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ServiceErr::ConnectionNotFound(err) => Some(err),
            ServiceErr::DbActionFailed(err) => Some(err),
            ServiceErr::JWTCreationError(err) => Some(err),
            _ => None,
        }
    }
}

impl Display for ServiceErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ServiceErr::ConnectionNotFound(err) => format!("Db Connection not found: {}", err),
                ServiceErr::DbActionFailed(err) => format!("Db Action Failed: {}", err),
                ServiceErr::JWTCreationError(err) => format!("Could not create JWT: {}", err),
                ServiceErr::TokenExpiredError => "Token expired.".to_string(),
                ServiceErr::JWTokenError => "Invalid JWT".to_string(),
                ServiceErr::NotFound => "Not found".to_string(),
                ServiceErr::ActixError(err) => format!("Actix error: {}", err),
            }
        )
    }
}

impl ResponseError for ServiceErr {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceErr::TokenExpiredError => HttpResponse::Unauthorized().body("Token expired."),
            ServiceErr::JWTokenError => HttpResponse::BadRequest().body("Invalid JWT."),
            ServiceErr::NotFound => HttpResponse::NotFound().body("Not Found"),
            ServiceErr::ActixError(err) => ResponseError::error_response(err),
            err => HttpResponse::InternalServerError().body(format!("{}", err)),
        }
    }
}

impl From<actix_web::Error> for ServiceErr {
    fn from(err: actix_web::Error) -> Self {
        Self::ActixError(err)
    }
}

impl From<diesel::result::Error> for ServiceErr {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => Self::NotFound,
            _ => Self::DbActionFailed(err),
        }
    }
}

impl From<r2d2::Error> for ServiceErr {
    fn from(err: r2d2::Error) -> Self {
        Self::ConnectionNotFound(err)
    }
}

#[feature(try_trait)]
impl From<NoneError> for ServiceErr {
    fn from(_: NoneError) -> Self {
        ServiceErr::NotFound
    }
}
