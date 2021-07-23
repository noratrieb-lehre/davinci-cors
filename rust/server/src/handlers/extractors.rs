use crate::actions::Pool;
use crate::error::ServiceErr;
use crate::handlers::auth::{validate_token, Claims};
use crate::models::conversion::IntoDto;
use actix_web::dev::Payload;
use actix_web::http::header::Header;
use actix_web::{web, FromRequest, HttpRequest};
use actix_web_httpauth::headers::authorization;
use actix_web_httpauth::headers::authorization::Bearer;
use dto::MemberRole;
use jsonwebtoken::DecodingKey;
use std::future;
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use uuid::Uuid;

/// Extract the role of a member in a class
/// - Validate that a user belongs to a class
/// Also makes sure that a user is logged in
#[derive(Debug, Clone)]
pub struct Role(pub MemberRole);

impl Deref for Role {
    type Target = MemberRole;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        future::ready(Self::from_request_sync(req).map_err(|err| err.into()))
    }
}

impl Claims {
    /// The body of the fromRequest implementation, so it can be reused. (non-blocking, since it doesn't do any io)
    fn from_request_sync(req: &HttpRequest) -> Result<Self, ServiceErr> {
        let key = req
            .app_data::<DecodingKey>()
            .expect("no decoding key found");

        match authorization::Authorization::<Bearer>::parse(req) {
            Ok(auth) => validate_token(auth.into_scheme().token(), key),
            Err(_) => Err(ServiceErr::Unauthorized("auth/no-token")),
        }
        .and_then(|claims| match claims.refresh {
            true => Err(ServiceErr::Unauthorized(
                "A refresh token can't be used for authentication",
            )),
            false => Ok(claims),
        })
    }
}

impl FromRequest for Role {
    type Error = ServiceErr;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let db = req.app_data::<Pool>().expect("db pool in app data").clone();

        let class_id = req
            .match_info()
            .get("classid")
            .ok_or(ServiceErr::BadRequest("request/no-class-id"))
            .and_then(|id| uuid::Uuid::parse_str(id).map_err(|e| e.into()))
            .unwrap();

        let claims = Claims::from_request_sync(req);

        Box::pin(async move {
            get_member_role(db, Ok(class_id), claims)
                .await
                .map_err(|err| match err {
                    ServiceErr::NotFound => ServiceErr::Unauthorized("auth/no-access"),
                    err => err,
                })
        })
    }
}

async fn get_member_role(
    db: Pool,
    class_id: Result<Uuid, ServiceErr>,
    claims: Result<Claims, ServiceErr>,
) -> Result<Role, ServiceErr> {
    let claims = claims?;
    Ok(Role(if claims.uid.is_nil() {
        MemberRole::CORS
    } else {
        web::block(move || crate::actions::class::get_member(&db, claims.uid, class_id?))
            .await?
            .into_dto()?
            .role
    }))
}
