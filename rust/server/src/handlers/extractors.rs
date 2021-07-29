use crate::actions::Pool;
use crate::error::ServiceErr;
use crate::handlers::auth::Claims;
use crate::models::conversion::IntoDto;
use actix_web::dev::Payload;
use actix_web::{web, FromRequest, HttpRequest};
use dto::MemberRole;
use std::future;
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
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

impl FromRequest for Role {
    type Error = ServiceErr;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let db = req
            .app_data::<web::Data<Pool>>()
            .expect("db pool in app data")
            .clone()
            .into_inner();

        let class_id = req
            .match_info()
            .get("classid")
            .ok_or(ServiceErr::BadRequest("no-class-id"))
            .and_then(|id| uuid::Uuid::parse_str(id).map_err(|e| e.into()));

        let claims = Claims::from_request_sync(req);

        Box::pin(async move {
            get_member_role(db, class_id, claims)
                .await
                .map_err(|err| match err {
                    ServiceErr::NotFound => ServiceErr::Unauthorized("no-access"),
                    err => err,
                })
        })
    }
}

async fn get_member_role(
    db: Arc<Pool>,
    class_id: Result<Uuid, ServiceErr>,
    claims: Result<Claims, ServiceErr>,
) -> Result<Role, ServiceErr> {
    let claims = claims?;
    Ok(Role(if claims.uid.is_nil() {
        MemberRole::CORS
    } else {
        let role =
            web::block(move || crate::actions::class::get_member(&db, claims.uid, class_id?))
                .await?
                .0
                .role
                .into_dto()?;
        if MemberRole::Member < role {
            return Err(ServiceErr::Unauthorized("banned"));
        }
        role
    }))
}
