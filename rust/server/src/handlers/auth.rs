use crate::error::ServiceErr;
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header::Header;
use actix_web::{FromRequest, HttpRequest};
use actix_web_httpauth::headers::authorization;
use actix_web_httpauth::headers::authorization::Bearer;
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::future;
use uuid::Uuid;

/// The claims of the JWT
///
/// *Note: the claim extractor rejects valid JWTs, if their refresh field is set to `true`*
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// The expiration date of the token
    pub exp: i64,
    /// The user id
    pub uid: Uuid,
    /// If the field is true, the token can only be used to get another token
    pub refresh: bool,
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        future::ready(
            match authorization::Authorization::<Bearer>::parse(req) {
                Ok(auth) => validate_token(auth.into_scheme().token()),
                Err(_) => Err(ErrorUnauthorized("No Bearer token present")),
            }
            .and_then(|claims| match claims.refresh {
                true => Err(ErrorUnauthorized(
                    "A refresh token can't be used for authentication",
                )),
                false => Ok(claims),
            }),
        )
    }
}

pub fn validate_token(token: &str) -> Result<Claims, actix_web::Error> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET env var");

    let decoded = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|_| ServiceErr::JWTokenError)?
    .claims;

    if decoded.exp < Utc::now().timestamp() {
        Err(ServiceErr::TokenExpiredError.into())
    } else {
        Ok(decoded)
    }
}

pub fn create_normal_jwt(user: Uuid) -> Result<(String, i64), ServiceErr> {
    create_jwt(user, false)
}
pub fn create_refresh_jwt(user: Uuid) -> Result<(String, i64), ServiceErr> {
    create_jwt(user, true)
}

fn create_jwt(uid: Uuid, refresh: bool) -> Result<(String, i64), ServiceErr> {
    let lifetime = if refresh {
        chrono::Duration::weeks(10)
    } else {
        chrono::Duration::hours(1)
    };

    let exp = Utc::now()
        .checked_add_signed(lifetime)
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims { exp, uid, refresh };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET env var");

    let header = jsonwebtoken::Header::new(Algorithm::HS512);
    jsonwebtoken::encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map(|str| (str, exp))
    .map_err(ServiceErr::JWTCreationError)
}
