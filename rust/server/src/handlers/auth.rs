use crate::actions::{self, Pool};
use crate::error::ServiceErr;
use crate::handlers::HttpResult;
use crate::models;
use crate::models::conversion::IntoDto;
use crate::models::NewUser;
use actix_web::http::header::Header;
use actix_web::web::*;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_httpauth::headers::authorization;
use actix_web_httpauth::headers::authorization::Bearer;
use chrono::Utc;
use dto::{ChangePasswordReq, LoginResponse, PostUser, UserLogin, UserPostResponse};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use tracing::debug;
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
    /// 0 on Refresh tokens, non-null on normal tokens  
    /// The version of the token, must match the current version
    pub version: i32,
}

pub fn auth_config(cfg: &mut ServiceConfig) {
    cfg.route("/token", get().to(refresh_token))
        .route("/login", post().to(login))
        .route(
            "/get-bot-token/{JWTSECRET}",
            get().to(secret_get_bot_user_token),
        );
}

/// `/token`
async fn refresh_token(
    req: HttpRequest,
    e_key: Data<EncodingKey>,
    db: Data<Pool>,
    d_key: Data<DecodingKey<'static>>,
) -> HttpResult {
    let auth = authorization::Authorization::<Bearer>::parse(&req)
        .map_err(|_| ServiceErr::Unauthorized("no-token"))?;

    let claims = validate_token(auth.into_scheme().token(), &d_key)?;
    let uid = claims.uid;

    debug!(%uid, "refresh token");

    let user = block(move || actions::user::get_user_by_id(&db, uid)).await?;

    if claims.version != user.token_version {
        return Err(ServiceErr::Unauthorized("old-token"));
    }

    if claims.refresh {
        let new_token = create_normal_jwt(uid, &e_key)?;
        Ok(HttpResponse::Ok()
            .header("token", format!("Bearer {}", new_token.0))
            .json(dto::RefreshResponse {
                expires: new_token.1,
            }))
    } else {
        Err(ServiceErr::Unauthorized("wrong-token-kind"))
    }
}

async fn login(mut body: Json<UserLogin>, db: Data<Pool>, key: Data<EncodingKey>) -> HttpResult {
    // to make the logging safe - we don't want to leak passwords
    let password = std::mem::replace(&mut body.password, "**********".to_string());
    debug!(?body, "login");

    let user =
        block(move || actions::user::validate_user_password(&db, &body.email, &password)).await?;

    match user {
        Some(user) => {
            let refresh_token = create_refresh_jwt(user.id, &key, user.token_version)?;
            let (token, expires) = create_normal_jwt(user.id, &key)?;
            Ok(HttpResponse::Ok()
                .header("token", format!("Bearer {}", token))
                .header("refresh-token", format!("Bearer {}", refresh_token))
                .json(LoginResponse {
                    userid: user.id,
                    expires,
                }))
        }
        None => Ok(HttpResponse::Forbidden().body("invalid-email-password")),
    }
}

pub async fn create_user(
    mut body: Json<PostUser>,
    db: Data<Pool>,
    key: Data<EncodingKey>,
) -> HttpResult {
    // to make the logging safe - we don't want to leak passwords
    let password = std::mem::replace(&mut body.password, "**********".to_string());

    debug!(?body, "create a user");

    let user = block(move || {
        let new_user = NewUser {
            id: uuid::Uuid::new_v4(),
            email: &body.email,
            password: &password,
            description: &body.description,
            discord_id: None,
            token_version: 1,
        };

        actions::user::insert_user(&db, new_user)
    })
    .await?;

    let (token, expires) = create_normal_jwt(user.id, &key)?;
    let refresh_token = create_refresh_jwt(user.id, &key, 1)?;

    Ok(HttpResponse::Created()
        .header("Token", format!("Bearer {}", token))
        .header("Refresh-Token", format!("Bearer {}", refresh_token))
        .json(UserPostResponse {
            user: dto::User {
                id: user.id,
                email: user.email,
                description: user.description,
                classes: None,
            },
            expires,
        }))
}

pub async fn change_password(
    claims: Claims,
    db: Data<Pool>,
    e_key: Data<EncodingKey>,
    password: Json<ChangePasswordReq>,
) -> HttpResult {
    debug!(uid = %claims.uid, "change user password");

    let user = block(move || {
        let user = actions::user::get_user_by_id(&db, claims.uid)?;
        let validate =
            actions::user::validate_user_password(&db, &user.email, &password.old_password)?;

        if validate.is_none() {
            return Err(ServiceErr::Unauthorized("wrong-password"));
        }

        actions::user::change_user_password(
            &db,
            models::User {
                id: claims.uid,
                email: "".to_string(),
                password: password.into_inner().password,
                description: "".to_string(),
                discord_id: None,
                token_version: 0,
            },
        )?;

        actions::user::increment_token_version(&db, claims.uid)
    })
    .await?;

    let refresh_token = create_refresh_jwt(user.id, &e_key, user.token_version)?;

    Ok(HttpResponse::Ok()
        .header("Refresh-Token", format!("Bearer {}", refresh_token))
        .json(user.into_dto()?))
}

async fn secret_get_bot_user_token(token: Path<String>, e_key: Data<EncodingKey>) -> HttpResult {
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| ServiceErr::InternalServerError("Secret not found".to_string()))?;

    if *token == secret {
        let uuid = uuid::Uuid::nil();

        Ok(HttpResponse::Ok()
            .header(
                "Token",
                format!(
                    "Bearer {}",
                    create_bot_jwt(uuid, &e_key, chrono::Duration::weeks(10000))?
                ),
            )
            .finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

impl Claims {
    /// The body of the fromRequest implementation, so it can be reused. (non-blocking, since it doesn't do any io)
    pub fn from_request_sync(req: &HttpRequest) -> Result<Self, ServiceErr> {
        let key = req
            .app_data::<web::Data<DecodingKey>>()
            .expect("no decoding key found");

        match authorization::Authorization::<Bearer>::parse(req) {
            Ok(auth) => validate_token(auth.into_scheme().token(), key),
            Err(_) => Err(ServiceErr::Unauthorized("no-token")),
        }
        .and_then(|claims| match claims.refresh {
            true => Err(ServiceErr::Unauthorized("wrong-token-kind")),
            false => Ok(claims),
        })
    }
}

//////////
////////// Tokens
//////////

fn validate_token(token: &str, key: &DecodingKey) -> Result<Claims, ServiceErr> {
    let decoded = jsonwebtoken::decode::<Claims>(token, key, &Validation::new(Algorithm::HS512))
        .map_err(|_| ServiceErr::JWTokenError)?
        .claims;

    if decoded.exp < Utc::now().timestamp_millis() {
        Err(ServiceErr::TokenExpiredError)
    } else {
        Ok(decoded)
    }
}

/// Returns the token and the expiration date
/// Create a JWT
fn create_normal_jwt(user: Uuid, key: &EncodingKey) -> Result<(String, i64), ServiceErr> {
    let lifetime;

    // make the token last 24 hours for debugging
    #[cfg(debug_assertions)]
    {
        lifetime = chrono::Duration::hours(24);
    }
    #[cfg(not(debug_assertions))]
    {
        lifetime = chrono::Duration::hours(1);
    }
    create_jwt(user, false, key, lifetime, 0)
}

/// Create a refresh JWT
/// Returns the token and the expiration date
fn create_refresh_jwt(user: Uuid, key: &EncodingKey, version: i32) -> Result<String, ServiceErr> {
    let lifetime = chrono::Duration::weeks(1000); // several years, kind of a hack but ok

    create_jwt(user, true, key, lifetime, version).map(|(token, _)| token)
}

/// Create a custom expiration date jwt
/// Returns the token and the expiration date
fn create_bot_jwt(
    user: Uuid,
    key: &EncodingKey,
    time: chrono::Duration,
) -> Result<String, ServiceErr> {
    create_jwt(user, false, key, time, 0).map(|(token, _)| token)
}

fn create_jwt(
    uid: Uuid,
    refresh: bool,
    key: &EncodingKey,
    lifetime: chrono::Duration,
    version: i32,
) -> Result<(String, i64), ServiceErr> {
    let exp = Utc::now()
        .checked_add_signed(lifetime)
        .expect("valid timestamp")
        .timestamp_millis();

    let claims = Claims {
        exp,
        uid,
        refresh,
        version,
    };

    let header = jsonwebtoken::Header::new(Algorithm::HS512);
    jsonwebtoken::encode(&header, &claims, key)
        .map(|str| (str, exp))
        .map_err(ServiceErr::JWTCreationError)
}

#[cfg(test)]
mod test {
    use crate::actions::Pool;
    use crate::handlers::auth::{create_jwt, create_user, validate_token, Claims};
    use actix_web::dev::Payload;
    use actix_web::test::TestRequest;
    use actix_web::{web, FromRequest, HttpMessage};
    use jsonwebtoken::{DecodingKey, EncodingKey};

    #[test]
    fn create_and_validate_token() {
        let encoding_key = EncodingKey::from_secret(b"cooles secret");
        let decoding_key = DecodingKey::from_secret(b"cooles secret");
        let uid = uuid::Uuid::new_v4();

        let token = create_jwt(uid, false, &encoding_key, chrono::Duration::hours(1), 1).unwrap();

        assert!(token.1 > chrono::Utc::now().timestamp_millis());

        let decoded = validate_token(&token.0, &decoding_key).unwrap();
        assert_eq!(decoded.uid, uid);
        assert_eq!(decoded.version, 1);
        assert_eq!(decoded.refresh, false);
    }

    #[actix_rt::test]
    async fn deserialize_token_from_request() {
        let encoding_key = EncodingKey::from_secret(b"cooles secret");
        let decoding_key = DecodingKey::from_secret(b"cooles secret");
        let uid = uuid::Uuid::new_v4();

        let (token, _) =
            create_jwt(uid, false, &encoding_key, chrono::Duration::hours(1), 1).unwrap();

        let req = TestRequest::with_header("Authorization", format!("Bearer {}", token))
            .app_data(web::Data::new(decoding_key))
            .to_http_request();

        let claims = Claims::from_request(&req, &mut Payload::None)
            .await
            .unwrap();

        assert_eq!(claims.uid, uid);
        assert_eq!(claims.version, 1);
    }

    fn get_pool() -> Pool {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = r2d2::ConnectionManager::<diesel::pg::PgConnection>::new(database_url);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    }
}
