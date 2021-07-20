use actix_web::{HttpResponse, ResponseError};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ServiceErr {
    ConnectionNotFound(r2d2::Error),
    DbActionFailed(diesel::result::Error),
    NotFound(diesel::result::Error),
    JWTCreationError(jsonwebtoken::errors::Error),
    TokenExpiredError,
    JWTokenError,
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
                ServiceErr::TokenExpiredError => format!("Token expired."),
                ServiceErr::JWTokenError => format!("Invalid JWT"),
                ServiceErr::NotFound(err) => format!("Not found: {}", err),
            }
        )
    }
}

impl ResponseError for ServiceErr {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceErr::TokenExpiredError => HttpResponse::Unauthorized().body("Token expired."),
            ServiceErr::JWTokenError => HttpResponse::BadRequest().body("Invalid JWT."),
            ServiceErr::NotFound(_) => HttpResponse::NotFound().body("Not Found"),
            err => HttpResponse::InternalServerError().body(format!("{}", err)),
        }
    }
}

impl From<diesel::result::Error> for ServiceErr {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => Self::NotFound(err),
            _ => Self::DbActionFailed(err),
        }
    }
}

impl From<r2d2::Error> for ServiceErr {
    fn from(err: r2d2::Error) -> Self {
        Self::ConnectionNotFound(err)
    }
}
