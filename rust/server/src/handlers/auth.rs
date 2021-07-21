use crate::actions::{self, Pool};
use crate::error::ServiceErr;
use crate::handlers::HttpResult;
use actix_web::dev::Payload;
use actix_web::http::header::Header;
use actix_web::{web, FromRequest, HttpRequest, HttpResponse};
use actix_web_httpauth::headers::authorization;
use actix_web_httpauth::headers::authorization::Bearer;
use chrono::Utc;
use dao::{LoginResponse, UserLogin};
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

pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/token", web::get().to(refresh_token))
        .route("/login", web::post().to(login));
}

async fn refresh_token(req: HttpRequest) -> HttpResult {
    let claims = match authorization::Authorization::<Bearer>::parse(&req) {
        Ok(auth) => validate_token(auth.into_scheme().token()),
        Err(_) => Err(ServiceErr::Unauthorized("No Bearer token present")),
    }?;

    if claims.refresh {
        let new_token = create_normal_jwt(claims.uid)?;
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

async fn login(body: web::Json<UserLogin>, db: web::Data<Pool>) -> HttpResult {
    let user =
        web::block(move || actions::user::validate_user_password(&db, &body.email, &body.password))
            .await?;

    match user {
        Some(user) => {
            let refresh_token = create_refresh_jwt(user.id.clone())?;
            let (token, expires) = create_normal_jwt(user.id.clone())?;
            Ok(HttpResponse::Ok()
                .header("Token", token)
                .header("Refresh-Token", refresh_token)
                .json(LoginResponse {
                    userid: user.id,
                    expires,
                }))
        }
        None => Ok(HttpResponse::Forbidden().body("Incorrect email or password")),
    }
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        future::ready(
            match authorization::Authorization::<Bearer>::parse(req) {
                Ok(auth) => validate_token(auth.into_scheme().token()),
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

pub fn validate_token(token: &str) -> Result<Claims, ServiceErr> {
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

/// Returns the token and the expiration date
/// Create a JWT
pub fn create_normal_jwt(user: Uuid) -> Result<(String, i64), ServiceErr> {
    create_jwt(user, false)
}

/// Create a refresh JWT
/// Returns the token and the expiration date
pub fn create_refresh_jwt(user: Uuid) -> Result<String, ServiceErr> {
    create_jwt(user, true).map(|(token, _)| token)
}
fn create_jwt(uid: Uuid, refresh: bool) -> Result<(String, i64), ServiceErr> {
    let lifetime = if refresh {
        chrono::Duration::weeks(1000) // several years, kind of a hack but ok
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
