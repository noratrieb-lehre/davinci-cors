use crate::actions::Pool;
use crate::error::ServiceErr;
use crate::handlers::auth::{validate_token, Claims};
use crate::models::conversion::IntoDao;
use actix_web::dev::Payload;
use actix_web::http::header::Header;
use actix_web::{web, FromRequest, HttpRequest};
use actix_web_httpauth::headers::authorization;
use actix_web_httpauth::headers::authorization::Bearer;
use dao::MemberRole;
use jsonwebtoken::DecodingKey;
use std::future;
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PathUuid(Uuid);
#[derive(Debug, Clone)]
pub struct PathUuid2(Uuid, Uuid);

#[derive(Debug, Clone)]
pub struct Role(MemberRole);

impl Deref for PathUuid {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
    type Future = future::Ready<Result<Self, Self::Error>>;
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
    type Future = future::Ready<Result<Self, Self::Error>>;
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

impl FromRequest for Role {
    type Error = ServiceErr;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let extensions = req.extensions();
        let db = req.app_data::<Pool>().expect("db pool in app data").clone();
        let class_id = extensions
            .get::<PathUuid>()
            .cloned()
            .ok_or(ServiceErr::BadRequest("Class Id not provided".to_string()));
        let claims = extensions
            .get::<Claims>()
            .cloned()
            .ok_or(ServiceErr::Unauthorized("No Bearer Token present"));

        Box::pin(async move { get_member_role(db, class_id, claims).await })
    }
}

async fn get_member_role(
    db: Pool,
    user_id: Result<PathUuid, ServiceErr>,
    class_id: Result<Claims, ServiceErr>,
) -> Result<Role, ServiceErr> {
    Ok(Role(
        web::block(move || crate::actions::class::get_member(&db, *user_id?, class_id?.uid))
            .await?
            .into_dao()?
            .role,
    ))
}
