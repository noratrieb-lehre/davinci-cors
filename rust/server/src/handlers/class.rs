use crate::actions::{self, Pool};
use crate::error::ServiceErr;
use crate::handlers::auth::Claims;
use crate::handlers::extractors::Role;
use crate::handlers::HttpResult;
use crate::models::conversion::IntoDto;
use crate::models::{NewClass, NewMember};
use actix_web::{web, HttpResponse};
use dto::{Class, MemberRole};
use std::convert::TryInto;
use uuid::Uuid;

macro_rules! http_todo {
    () => {
        std::result::Result::Ok(actix_web::HttpResponse::Ok().body("Unimplemented"))
    };
}

pub(super) fn class_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/classes", web::post().to(create_class)).service(
        web::scope("/classes/{class_id}")
            .route("", web::get().to(get_class))
            .route("", web::put().to(edit_class))
            .route("", web::delete().to(delete_class))
            .route("/members/{uuid}", web::put().to(edit_member))
            .route("/join", web::post().to(request_join))
            .route("/requests", web::get().to(get_join_requests))
            .route("/requests/{uuid}", web::post().to(accept_member))
            .route("/events", web::get().to(get_events))
            .route("/events", web::post().to(create_event))
            .route("/events/{uuid}", web::get().to(get_event))
            .route("/events/{uuid}", web::put().to(edit_event))
            .route("/events/{uuid}", web::delete().to(delete_event))
            .route("timetable", web::get().to(get_timetable))
            .route("timetable", web::put().to(edit_timetable)),
    );
}

async fn get_class(
    params: web::Path<Uuid>,
    db: web::Data<Pool>,
    claims: Claims,
    role: Role,
) -> HttpResult {
    //let uuid = uuid::Uuid::from_str(&params)?;
    //
    //let class = web::block(move || actions::class::get_class(&db, uuid))
    //    .await?
    //    .ok_or(ServiceErr::NotFound)?
    //    .into_dto()?;
    //
    //if class.members.iter().any(|member| member.user == claims.uid) {
    //    Ok(HttpResponse::Ok().json(class))
    //} else {
    //    Err(ServiceErr::Unauthorized("Cannot access other class"))
    //}

    Ok(HttpResponse::Ok().body(format!(
        "hallo class {:?} role {:?}",
        params.into_inner(),
        role
    )))
}

async fn create_class(class: web::Json<Class>, db: web::Data<Pool>, claims: Claims) -> HttpResult {
    let id = uuid::Uuid::new_v4();
    let uid = claims.uid;
    let class = class.into_inner();

    let new_class = NewClass {
        id,
        owner: claims.uid,
        name: class.name,
        description: class.description,
    };

    let (result_class, owner) = web::block::<_, _, ServiceErr>(move || {
        let class = actions::class::insert_class(&db, new_class)?;
        let user = actions::user::get_user_by_id(&db, uid)?;
        let new_member = NewMember {
            user: user.id,
            class: class.id,
            display_name: user.email.clone(),
            role: 0, // OWNER
        };
        let member = actions::class::create_member(&db, new_member)?;
        Ok((class, member))
    })
    .await?;

    let mut result_class = result_class.into_dto()?;

    result_class.members = vec![owner.into_dto()?];
    Ok(HttpResponse::Created().json(result_class))
}

async fn edit_class(
    class_id: web::Path<Uuid>,
    new_class: web::Json<Class>,
    db: web::Data<Pool>,
    claims: Claims,
) -> HttpResult {
    let class = web::block(move || {
        let member =
            actions::class::get_member(&db, claims.uid, class_id.into_inner())?.into_dto()?;

        if member.role.has_rights() {
            actions::class::update_class(&db, new_class.into_inner().try_into()?)
        } else {
            Err(ServiceErr::Unauthorized("Is not an Administrator"))
        }
    })
    .await?
    .into_dto()?;

    Ok(HttpResponse::Ok().json(class))
}

async fn delete_class(
    class_id: web::Path<Uuid>,
    db: web::Data<Pool>,
    claims: Claims,
) -> HttpResult {
    let deleted_amount = web::block(move || {
        let member = actions::class::get_member(&db, claims.uid, class_id.clone())?.into_dto()?;

        if member.role == MemberRole::Owner {
            Ok(actions::class::delete_class(&db, *class_id)?)
        } else {
            Err(ServiceErr::Unauthorized("Is not the Owner"))
        }
    })
    .await?;

    Ok(match deleted_amount {
        0 => HttpResponse::NotFound().body("Class not found"),
        1 => HttpResponse::Ok().body("Deleted class."),
        _ => unreachable!(),
    })
}

async fn edit_member(role: Role) -> HttpResult {
    Ok(HttpResponse::Ok().body(format!("hallo role {:?}", role)))
}

async fn request_join() -> HttpResult {
    http_todo!()
}

async fn get_join_requests() -> HttpResult {
    http_todo!()
}
async fn accept_member() -> HttpResult {
    http_todo!()
}

async fn get_event() -> HttpResult {
    http_todo!()
}

async fn get_events() -> HttpResult {
    http_todo!()
}

async fn create_event() -> HttpResult {
    http_todo!()
}

async fn edit_event() -> HttpResult {
    http_todo!()
}

async fn delete_event() -> HttpResult {
    http_todo!()
}

async fn get_timetable() -> HttpResult {
    http_todo!()
}

async fn edit_timetable() -> HttpResult {
    http_todo!()
}
