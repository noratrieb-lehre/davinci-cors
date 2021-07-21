use actix_web::error::BlockingError;
use actix_web::{HttpResponse, ResponseError};
use std::error::Error;
use std::fmt::{Display, Formatter};

pub type ServiceResult<T> = Result<T, ServiceErr>;

#[derive(Debug)]
pub enum ServiceErr {
    BadRequest(String),
    InternalServerError(String),
    ConnectionNotFound(r2d2::Error),
    DbActionFailed(diesel::result::Error),
    NotFound,
    JWTCreationError(jsonwebtoken::errors::Error),
    TokenExpiredError,
    JWTokenError,
    Unauthorized(&'static str),
    InvalidDao(String),
    Conflict(String),
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
                ServiceErr::InternalServerError(msg) => format!("Internal Server Error: {}", msg),
                ServiceErr::Unauthorized(msg) => format!("Unauthorized: {}", msg),
                ServiceErr::InvalidDao(msg) => format!("Invalid Data for DAO: {}", msg),
                ServiceErr::BadRequest(msg) => format!("Bad Request: {}", msg),
                ServiceErr::Conflict(msg) => format!("Conflict: {}", msg),
            }
        )
    }
}

impl ResponseError for ServiceErr {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceErr::TokenExpiredError => HttpResponse::Unauthorized().body("Token expired."),
            ServiceErr::JWTokenError => HttpResponse::BadRequest().body("Invalid JWT."),
            ServiceErr::BadRequest(msg) => HttpResponse::BadRequest().body(msg),
            ServiceErr::NotFound => HttpResponse::NotFound().body("Not Found"),
            ServiceErr::Unauthorized(msg) => HttpResponse::Unauthorized().body(*msg),
            ServiceErr::Conflict(msg) => HttpResponse::Conflict().body(msg),
            err => HttpResponse::InternalServerError().body(format!("{}", err)),
        }
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

impl From<uuid::Error> for ServiceErr {
    fn from(_: uuid::Error) -> Self {
        Self::BadRequest("Could not create UUID".to_string())
    }
}

impl<E: std::fmt::Debug> From<BlockingError<E>> for ServiceErr
where
    Self: From<E>,
{
    fn from(err: BlockingError<E>) -> Self {
        match err {
            BlockingError::Error(inner) => inner.into(),
            BlockingError::Canceled => Self::InternalServerError("Thread pool is gone".to_string()),
        }
    }
}
