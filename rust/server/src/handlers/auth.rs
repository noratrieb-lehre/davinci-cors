use crate::actions::{self, Pool};
use crate::error::ServiceErr;
use crate::handlers::HttpResult;
use actix_web::http::header::Header;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_httpauth::headers::authorization;
use actix_web_httpauth::headers::authorization::Bearer;
use chrono::Utc;
use dto::{LoginResponse, UserLogin};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
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

#[allow(clippy::needless_lifetimes)] // borrow checker too dumb to get this
async fn refresh_token<'a>(
    req: HttpRequest,
    e_key: web::Data<EncodingKey>,
    d_key: web::Data<DecodingKey<'a>>,
) -> HttpResult {
    let claims = match authorization::Authorization::<Bearer>::parse(&req) {
        Ok(auth) => validate_token(auth.into_scheme().token(), &d_key),
        Err(_) => Err(ServiceErr::Unauthorized("auth/no-token")),
    }?;

    if claims.refresh {
        let new_token = create_normal_jwt(claims.uid, &e_key)?;
        Ok(HttpResponse::Ok()
            .header("Token", new_token.0)
            .json(dto::RefreshResponse {
                expires: new_token.1,
            }))
    } else {
        Err(ServiceErr::Unauthorized(
            "Normal token cannot be used to get a new token",
        ))
    }
}

async fn login(
    body: web::Json<UserLogin>,
    db: web::Data<Pool>,
    key: web::Data<EncodingKey>,
) -> HttpResult {
    let user =
        web::block(move || actions::user::validate_user_password(&db, &body.email, &body.password))
            .await?;

    match user {
        Some(user) => {
            let refresh_token = create_refresh_jwt(user.id, &key)?;
            let (token, expires) = create_normal_jwt(user.id, &key)?;
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

pub fn validate_token(token: &str, key: &DecodingKey) -> Result<Claims, ServiceErr> {
    let decoded = jsonwebtoken::decode::<Claims>(&token, key, &Validation::new(Algorithm::HS512))
        .map_err(|_| ServiceErr::JWTokenError)?
        .claims;

    if decoded.exp < Utc::now().timestamp() * 1000 {
        Err(ServiceErr::TokenExpiredError)
    } else {
        Ok(decoded)
    }
}

/// Returns the token and the expiration date
/// Create a JWT
pub fn create_normal_jwt(user: Uuid, key: &EncodingKey) -> Result<(String, i64), ServiceErr> {
    create_jwt(user, false, key)
}

/// Create a refresh JWT
/// Returns the token and the expiration date
pub fn create_refresh_jwt(user: Uuid, key: &EncodingKey) -> Result<String, ServiceErr> {
    create_jwt(user, true, key).map(|(token, _)| token)
}
fn create_jwt(uid: Uuid, refresh: bool, key: &EncodingKey) -> Result<(String, i64), ServiceErr> {
    let lifetime;
    if refresh {
        lifetime = chrono::Duration::weeks(1000) // several years, kind of a hack but ok
    } else {
        // make the token last 24 hours for debugging
        #[cfg(debug_assertions)]
        {
            lifetime = chrono::Duration::hours(24);
        }
        #[cfg(not(debug_assertions))]
        {
            lifetime = chrono::Duration::hours(1);
        }
    };

    let exp = Utc::now()
        .checked_add_signed(lifetime)
        .expect("valid timestamp")
        .timestamp()
        * 1000;

    let claims = Claims { exp, uid, refresh };

    let header = jsonwebtoken::Header::new(Algorithm::HS512);
    jsonwebtoken::encode(&header, &claims, key)
        .map(|str| (str, exp))
        .map_err(ServiceErr::JWTCreationError)
}
