use crate::actions::{self, Pool};
use crate::error::ServiceErr;
use crate::handlers::auth::Claims;
use crate::handlers::extractors::Role;
use crate::handlers::HttpResult;
use crate::models;
use crate::models::conversion::IntoDto;
use crate::models::{NewClass, NewEvent, NewMember, PENDING};
use actix_web::web::{block, delete, get, post, put, scope, Data, Json, Path, ServiceConfig};
use actix_web::{web, HttpResponse};
use dto::{Class, Event, Member, MemberAcceptDto, MemberRole, Snowflake, Timetable};
use uuid::Uuid;

pub(super) fn class_config(cfg: &mut ServiceConfig) {
    cfg.route("/classes", post().to(create_class)).service(
        scope("/classes/{classid}")
            .route("", get().to(get_class))
            .route("", put().to(edit_class))
            .route("", delete().to(delete_class))
            .route("/members/{uuid}", put().to(edit_member))
            .route("/join", post().to(request_join))
            .route("/requests", get().to(get_join_requests))
            .route("/requests/{uuid}", post().to(accept_member))
            .route("/events", get().to(get_events))
            .route("/events", post().to(create_event))
            .route("/events/{uuid}", get().to(get_event))
            .route("/events/{uuid}", put().to(edit_event))
            .route("/events/{uuid}", delete().to(delete_event))
            .route("/timetable", get().to(get_timetable))
            .route("/timetable", put().to(edit_timetable))
            .route("/link", post().to(link_class_with_discord))
            .route("/discord/{snowflake}", get().to(get_class_by_discord)),
    );
}

async fn get_class(class_path: Path<Uuid>, db: Data<Pool>, _role: Role) -> HttpResult {
    let class = block(move || actions::class::get_class(&db, class_path.into_inner()))
        .await?
        .ok_or(ServiceErr::NotFound)?
        .into_dto()?;

    Ok(HttpResponse::Ok().json(class))
}

async fn create_class(class: Json<Class>, db: Data<Pool>, claims: Claims) -> HttpResult {
    let (result_class, owner) = block::<_, _, ServiceErr>(move || {
        let class_id = uuid::Uuid::new_v4();

        let new_class = NewClass {
            id: class_id,
            owner: claims.uid,
            name: &class.name,
            description: &class.description,
            discord_id: None,
        };

        let class = actions::class::insert_class(&db, new_class)?;
        let user = actions::user::get_user_by_id(&db, claims.uid)?;
        let new_member = NewMember {
            user: user.id,
            class: class.id,
            display_name: &user.email,
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
    class_id: Path<Uuid>,
    new_class: Json<Class>,
    db: Data<Pool>,
    role: Role,
) -> HttpResult {
    if !role.has_rights() {
        return Err(ServiceErr::NoAdminPermissions);
    }

    let class = block(move || {
        let update_class = NewClass {
            id: class_id.into_inner(),
            owner: Default::default(), // doesn't matter
            name: &new_class.name,
            description: &new_class.description,
            discord_id: None,
        };

        actions::class::update_class(&db, update_class)
    })
    .await?
    .into_dto()?;

    Ok(HttpResponse::Ok().json(class))
}

async fn delete_class(class_id: Path<Uuid>, db: Data<Pool>, role: Role) -> HttpResult {
    if *role != MemberRole::Owner {
        return Err(ServiceErr::Unauthorized("auth/no-owner"));
    }

    let deleted_amount =
        block::<_, _, ServiceErr>(move || Ok(actions::class::delete_class(&db, *class_id)))
            .await??;

    Ok(match deleted_amount {
        0 => HttpResponse::NotFound().body("Class not found"),
        1 => HttpResponse::Ok().body("Deleted class."),
        _ => unreachable!(),
    })
}

async fn edit_member(
    path: Path<(Uuid, Uuid)>,
    role: Role,
    member: Json<Member>,
    db: Data<Pool>,
    claims: Claims,
) -> HttpResult {
    let (class_id, member_id) = path.into_inner();

    // Only admins can edit others
    if claims.uid != member_id && !role.has_rights() {
        return Err(ServiceErr::NoAdminPermissions);
    }

    // Cannot edit self
    if claims.uid == member.user {
        return Err(ServiceErr::BadRequest("auth/edit-own-permission"));
    }

    // Can only set target permissions lower than own
    if member.role >= *role {
        return Err(ServiceErr::BadRequest("auth/not-enough-permissions"));
    }

    let member = block(move || {
        let (old_member, _) = actions::class::get_member(&db, member_id, class_id)?;

        // Can only edit members lower than self
        if old_member.role >= role.0 as i32 {
            return Err(ServiceErr::BadRequest("auth/not-enough-permissions"));
        }

        let member = NewMember {
            user: member_id,
            class: class_id,
            display_name: &member.display_name,
            role: crate::models::conversion::member_role_dto_to_int(&member.role),
        };
        actions::class::update_member(&db, member)
    })
    .await?
    .into_dto()?;

    Ok(HttpResponse::Ok().json(member))
}

async fn request_join(class_id: Path<Uuid>, claims: Claims, db: Data<Pool>) -> HttpResult {
    block(move || {
        let user = actions::user::get_user_by_id(&db, claims.uid)?;
        let member = NewMember {
            user: claims.uid,
            class: *class_id,
            display_name: &user.email,
            role: PENDING,
        };

        actions::class::create_member(&db, member)
    })
    .await?;

    Ok(HttpResponse::Created().body("Pending response..."))
}

async fn get_join_requests(class_id: Path<Uuid>, role: Role, db: Data<Pool>) -> HttpResult {
    if !role.has_rights() {
        return Err(ServiceErr::NoAdminPermissions);
    }

    let members = block(move || actions::class::get_pending_members(&db, class_id.into_inner()))
        .await?
        .into_dto()?;

    Ok(HttpResponse::Ok().json(members))
}

async fn accept_member(
    path: Path<(Uuid, Uuid)>,
    role: Role,
    db: Data<Pool>,
    accept: Json<MemberAcceptDto>,
) -> HttpResult {
    let (class_id, member_id) = path.into_inner();

    if !role.has_rights() {
        return Err(ServiceErr::NoAdminPermissions);
    }

    let response = block(move || {
        if accept.accept {
            let (member, _) = actions::class::get_member(&db, member_id, class_id)?;
            if member.role != PENDING {
                return Err(ServiceErr::BadRequest("class/member-not-pending"));
            }
            let new_member = NewMember {
                user: member_id,
                class: class_id,
                display_name: &member.display_name,
                role: 2,
            };
            actions::class::update_member(&db, new_member)?;
            Ok("Accepted member.")
        } else {
            let deleted = actions::class::delete_member(&db, member_id, class_id)?;
            match deleted {
                0 => Err(ServiceErr::NotFound),
                1 => Ok("Denied member."),
                _ => unreachable!(),
            }
        }
    })
    .await?;

    Ok(HttpResponse::Ok().body(response))
}

async fn get_event(path: Path<(String, Uuid)>, _role: Role, db: Data<Pool>) -> HttpResult {
    let event = block(move || actions::event::get_event_by_id(&db, path.1))
        .await?
        .into_dto()?;

    Ok(HttpResponse::Ok().json(event))
}

async fn get_events(class_id: Path<Uuid>, _role: Role, db: Data<Pool>) -> HttpResult {
    // todo parameter
    let events = block(move || actions::event::get_events_by_class(&db, *class_id))
        .await?
        .into_dto()?;

    Ok(HttpResponse::Ok().json(events))
}

async fn create_event(
    class_id: Path<Uuid>,
    role: Role,
    db: Data<Pool>,
    event: Json<Event>,
) -> HttpResult {
    if !role.has_rights() {
        return Err(ServiceErr::NoAdminPermissions);
    }

    let event = block(move || {
        let new_event = NewEvent {
            id: uuid::Uuid::new_v4(),
            class: *class_id,
            e_type: event.r#type as i32,
            name: &event.name,
            start: &chrono::NaiveDateTime::from_timestamp(event.start / 1000, 0),
            end: &chrono::NaiveDateTime::from_timestamp(event.end / 1000, 0),
            description: &event.description,
        };

        actions::event::insert_event(&db, new_event)
    })
    .await?
    .into_dto()?;

    Ok(HttpResponse::Created().json(event))
}

async fn edit_event(
    path: Path<(Uuid, Uuid)>,
    role: Role,
    db: Data<Pool>,
    event: Json<Event>,
) -> HttpResult {
    if !role.has_rights() {
        return Err(ServiceErr::NoAdminPermissions);
    }

    let event = block(move || {
        let new_event = NewEvent {
            id: path.1,
            class: path.0,
            e_type: event.r#type as i32,
            name: &event.name,
            start: &chrono::NaiveDateTime::from_timestamp(event.start / 1000, 0),
            end: &chrono::NaiveDateTime::from_timestamp(event.end / 1000, 0),
            description: &event.description,
        };

        actions::event::update_event(&db, new_event)
    })
    .await?
    .into_dto()?;

    Ok(HttpResponse::Ok().json(event))
}

async fn delete_event(path: Path<(String, Uuid)>, role: Role, db: Data<Pool>) -> HttpResult {
    if !role.has_rights() {
        return Err(ServiceErr::NoAdminPermissions);
    }

    let deleted = block(move || actions::event::delete_event(&db, path.1)).await?;

    Ok(match deleted {
        0 => HttpResponse::NotFound().body("Event not found"),
        1 => HttpResponse::Ok().body("Deleted event."),
        _ => unreachable!(),
    })
}

async fn get_timetable(path: Path<Uuid>, _role: Role, db: Data<Pool>) -> HttpResult {
    let timetable = block(move || actions::class::get_timetable(&db, *path))
        .await?
        .timetable;

    Ok(HttpResponse::Ok().json(timetable))
}

async fn edit_timetable(
    path: Path<Uuid>,
    role: Role,
    db: Data<Pool>,
    table: Json<Timetable>,
) -> HttpResult {
    if !role.has_rights() {
        return Err(ServiceErr::NoAdminPermissions);
    }

    let timetable = block(move || {
        actions::class::update_timetable(
            &db,
            models::Timetable {
                class: *path,
                timetable: serde_json::to_string(&table.into_inner()).map_err(|_| {
                    ServiceErr::InternalServerError("serialize-timetable".to_string())
                })?,
            },
        )
    })
    .await?
    .timetable;

    Ok(HttpResponse::Ok().json(timetable))
}

async fn link_class_with_discord(
    class_id: Path<Uuid>,
    role: Role,
    db: Data<Pool>,
    id: Json<Snowflake>,
) -> HttpResult {
    if *role != MemberRole::Owner {
        return Err(ServiceErr::BadRequest("auth/no-owner"));
    }

    let class = block(move || {
        actions::class::set_discord_id_class(&db, *class_id, Some(id.into_inner().snowflake))
    })
    .await?
    .into_dto()?;

    Ok(HttpResponse::Ok().json(class))
}

async fn get_class_by_discord(
    class_id: Path<String>,
    claims: Claims,
    db: Data<Pool>,
) -> HttpResult {
    if !claims.uid.is_nil() {
        return Err(ServiceErr::NotFound); // very secret route
    }

    let class = block(move || actions::class::get_class_by_discord(&db, &class_id))
        .await?
        .into_dto()?;

    Ok(HttpResponse::Ok().json(class))
}
