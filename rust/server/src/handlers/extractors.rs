use crate::error::ServiceErr;
use crate::handlers::auth::{validate_token, Claims};
use crate::handlers::{PathUuid, PathUuid2};
use actix_web::dev::Payload;
use actix_web::http::header::Header;
use actix_web::{web, FromRequest, HttpRequest};
use actix_web_httpauth::headers::authorization;
use actix_web_httpauth::headers::authorization::Bearer;
use jsonwebtoken::DecodingKey;
use std::future;
use uuid::Uuid;

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let key = req
            .app_data::<DecodingKey>()
            .expect("no decoding key found");

        future::ready(
            match authorization::Authorization::<Bearer>::parse(req) {
                Ok(auth) => validate_token(auth.into_scheme().token(), key),
                Err(_) => Err(ServiceErr::Unauthorized("No Bearer token present")),
            }
            .and_then(|claims| match claims.refresh {
                true => Err(ServiceErr::Unauthorized(
                    "A refresh token can't be used for authentication",
                )),
                false => Ok(claims),
            })
            .map_err(|err| err.into()),
        )
    }
}

impl FromRequest for PathUuid {
    type Error = ServiceErr;
    type Future = std::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let extensions = req.extensions();

        future::ready(match extensions.get::<web::Path<String>>() {
            None => Err(ServiceErr::NotFound),
            Some(path) => uuid::Uuid::parse_str(&path)
                .map_err(|err| err.into())
                .map(|uuid| PathUuid(uuid)),
        })
    }
}

impl FromRequest for PathUuid2 {
    type Error = ServiceErr;
    type Future = std::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let extensions = req.extensions();

        future::ready(match extensions.get::<web::Path<(String, String)>>() {
            None => Err(ServiceErr::NotFound),
            Some(path) => match (Uuid::parse_str(&path.0), Uuid::parse_str(&path.1)) {
                (Ok(v), Ok(v2)) => Ok((v, v2)),
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            }
            .map_err(|err| err.into())
            .map(|(uuid1, uuid2)| PathUuid2(uuid1, uuid2)),
        })
    }
}
