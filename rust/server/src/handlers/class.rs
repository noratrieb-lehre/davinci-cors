use crate::actions::{self, Pool};
use crate::error::ServiceErr;
use crate::handlers::auth::Claims;
use crate::handlers::extractors::Role;
use crate::handlers::HttpResult;
use crate::models;
use crate::models::conversion::IntoDto;
use crate::models::{NewClass, NewEvent, NewGuild, NewMember, PENDING};
use actix_web::web::{
    block, delete, get, post, put, scope, Data, Json, Path, Query, ServiceConfig,
};
use actix_web::HttpResponse;
use chrono::NaiveDateTime;
use dto::{
    Class, Event, GetEventQueryParams, Guild, Member, MemberAcceptDto, MemberRole, SingleSnowflake,
    Timetable,
};
use tracing::debug;
use uuid::Uuid;

pub(super) fn class_config(cfg: &mut ServiceConfig) {
    cfg.route("/classes", post().to(create_class))
        .route(
            "/classes/discord/{snowflake}",
            get().to(get_class_by_discord),
        )
        .route("/bot/guilds", put().to(edit_guild_settings))
        .route("/bot/guilds/{snowflake}", get().to(get_guild))
        .service(
            scope("/classes/{classid}")
                .route("", get().to(get_class))
                .route("", put().to(edit_class))
                .route("", delete().to(delete_class))
                .route("/members/{uuid}", get().to(get_member))
                .route("/members/{uuid}", put().to(edit_member))
                .route("/members/{uuid}", delete().to(delete_member))
                .route("/bans", get().to(get_bans))
                .route("/join", post().to(request_join))
                .route("/requests", get().to(get_join_requests))
                .route("/requests/{uuid}", post().to(accept_member))
                .route("/events", get().to(get_events))
                .route("/events", post().to(create_event))
                .route("/events/{uuid}", get().to(get_event))
                .route("/events/{uuid}", put().to(edit_event))
                .route("/events/{uuid}", delete().to(delete_event))
                .route("/timetable", get().to(get_timetable))
                .route("/timetable", post().to(create_timetable))
                .route("/timetable", delete().to(delete_timetable))
                .route("/timetable", put().to(edit_timetable))
                .route("/link", post().to(link_class_with_discord)),
        );
}

async fn get_class(class_path: Path<Uuid>, db: Data<Pool>, _role: Role) -> HttpResult {
    debug!(%class_path, "get class");
    let class = block(move || actions::class::get_class(&db, class_path.into_inner()))
        .await?
        .ok_or(ServiceErr::NotFound)?
        .into_dto()?;

    Ok(HttpResponse::Ok().json(class))
}

async fn create_class(class: Json<Class>, db: Data<Pool>, claims: Claims) -> HttpResult {
    debug!(?class, userid = %claims.uid, "create a new class");

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
    debug!(%class_id, ?role, "edit class");

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
    debug!(%class_id, ?role, "delete class");

    if *role != MemberRole::Owner {
        return Err(ServiceErr::Unauthorized("no-owner"));
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

async fn get_member(path: Path<(Uuid, Uuid)>, _role: Role, db: Data<Pool>) -> HttpResult {
    let (class_id, member_id) = path.into_inner();
    debug!(%class_id, %member_id, ?_role, "get member");

    let member = block(move || actions::class::get_member(&db, member_id, class_id))
        .await?
        .into_dto()?;

    Ok(HttpResponse::Ok().json(member))
}

async fn edit_member(
    path: Path<(Uuid, Uuid)>,
    own_role: Role,
    member: Json<Member>,
    db: Data<Pool>,
    claims: Claims,
) -> HttpResult {
    let (class_id, member_id) = path.into_inner();

    debug!(%class_id, %member_id, ?own_role, userid = %claims.uid, ?member, "edit member");

    let member = block(move || {
        let (old_member, _) = actions::class::get_member(&db, member_id, class_id)?;

        // if edit other member
        if claims.uid != member_id {
            // Only admins can edit others
            if !own_role.has_rights() {
                return Err(ServiceErr::NoAdminPermissions);
            }

            // Can only set target permissions lower than own
            if member.role <= *own_role {
                return Err(ServiceErr::Unauthorized("not-enough-permissions"));
            }
            // Can only edit members lower than self
            if old_member.role <= own_role.0 as i32 {
                return Err(ServiceErr::Unauthorized("not-enough-permissions"));
            }
        }

        // Cannot edit own roles
        let new_role = if claims.uid == member.user {
            old_member.role
        } else {
            crate::models::conversion::member_role_dto_to_int(&member.role)
        };

        let member = NewMember {
            user: member_id,
            class: class_id,
            display_name: &member.display_name,
            role: new_role,
        };
        actions::class::update_member(&db, member)
    })
    .await?
    .into_dto()?;

    Ok(HttpResponse::Ok().json(member))
}

async fn delete_member(
    path: Path<(Uuid, Uuid)>,
    role: Role,
    db: Data<Pool>,
    claims: Claims,
) -> HttpResult {
    let (class_id, member_id) = path.into_inner();

    debug!(%class_id, %member_id, ?role, userid = %claims.uid, "delete member");

    // Must be admin to delete others
    if !role.has_rights() && claims.uid != member_id {
        return Err(ServiceErr::NoAdminPermissions);
    }

    // Class must always have an owner
    if claims.uid == member_id && *role == MemberRole::Owner {
        return Err(ServiceErr::BadRequest("must-have-owner"));
    }

    let deleted_amount = block(move || {
        let (old_member, _) = actions::class::get_member(&db, member_id, class_id)?;

        // Can only edit members lower than self
        if old_member.role <= role.0 as i32 {
            return Err(ServiceErr::Unauthorized("not-enough-permissions"));
        }

        actions::class::delete_member(&db, member_id, class_id)
    })
    .await?;

    match deleted_amount {
        0 => Err(ServiceErr::NotFound),
        1 => Ok(HttpResponse::Ok().body("Deleted member")),
        _ => unreachable!(),
    }
}

async fn get_bans(class_id: Path<Uuid>, role: Role, db: Data<Pool>) -> HttpResult {
    debug!(%class_id, ?role, "get ban requests");

    if !role.has_rights() {
        return Err(ServiceErr::NoAdminPermissions);
    }

    let bans = block(move || actions::class::get_banned_members(&db, class_id.into_inner()))
        .await?
        .into_dto()?;

    Ok(HttpResponse::Ok().json(bans))
}

async fn request_join(class_id: Path<Uuid>, claims: Claims, db: Data<Pool>) -> HttpResult {
    debug!(%class_id, userid = %claims.uid, "request join");

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
    debug!(%class_id, ?role, "get join requests");

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

    debug!(%class_id, %member_id, ?role, ?accept, "accept/deny member");

    if !role.has_rights() {
        return Err(ServiceErr::NoAdminPermissions);
    }

    let response = block(move || {
        if accept.accept {
            let (member, _) = actions::class::get_member(&db, member_id, class_id)?;
            if member.role != PENDING {
                return Err(ServiceErr::BadRequest("member-not-pending"));
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
    debug!(event_id = %path.1, ?_role, "get event");

    let event = block(move || actions::event::get_event_by_id(&db, path.1))
        .await?
        .into_dto()?;

    Ok(HttpResponse::Ok().json(event))
}

async fn get_events(
    class_id: Path<Uuid>,
    _role: Role,
    db: Data<Pool>,
    query: Query<GetEventQueryParams>,
) -> HttpResult {
    let GetEventQueryParams { before, after } = query.into_inner();

    debug!(%class_id, ?_role, ?before, ?after, "get events");

    let before = before.map(|b| b / 1000);
    let after = after.map(|a| a / 1000);

    let events = block(move || match (before, after) {
        (None, None) => actions::event::get_events_by_class(&db, *class_id),
        (Some(before), Some(after)) => actions::event::get_events_by_class_filtered_both(
            &db,
            *class_id,
            NaiveDateTime::from_timestamp(before, 0),
            NaiveDateTime::from_timestamp(after, 0),
        ),
        (Some(before), None) => actions::event::get_events_by_class_filtered_before(
            &db,
            *class_id,
            NaiveDateTime::from_timestamp(before, 0),
        ),
        (None, Some(after)) => actions::event::get_events_by_class_filtered_after(
            &db,
            *class_id,
            NaiveDateTime::from_timestamp(after, 0),
        ),
    })
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
    debug!(%class_id, ?role, ?event, "create event");

    if !role.has_rights() {
        return Err(ServiceErr::NoAdminPermissions);
    }

    let event = block(move || {
        let end = event
            .end
            .map(|ts| chrono::NaiveDateTime::from_timestamp(ts / 1000, 0));
        let notification = event
            .notification
            .map(|ts| chrono::NaiveDateTime::from_timestamp(ts / 1000, 0));

        let new_event = NewEvent {
            id: uuid::Uuid::new_v4(),
            class: *class_id,
            e_type: event.r#type as i32,
            name: &event.name,
            start: &chrono::NaiveDateTime::from_timestamp(event.start / 1000, 0),
            end: end.as_ref(),
            description: &event.description,
            notification: notification.as_ref(),
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
    let (class_id, event_id) = path.into_inner();

    debug!(%class_id, %event_id, ?role, ?event, "edit event");

    if !role.has_rights() {
        return Err(ServiceErr::NoAdminPermissions);
    }

    let event = block(move || {
        let end = event
            .end
            .map(|ts| chrono::NaiveDateTime::from_timestamp(ts / 1000, 0));
        let notification = event
            .notification
            .map(|ts| chrono::NaiveDateTime::from_timestamp(ts / 1000, 0));
        let new_event = NewEvent {
            id: event_id,
            class: class_id,
            e_type: event.r#type as i32,
            name: &event.name,
            start: &chrono::NaiveDateTime::from_timestamp(event.start / 1000, 0),
            end: end.as_ref(),
            description: &event.description,
            notification: notification.as_ref(),
        };

        actions::event::update_event(&db, new_event)
    })
    .await?
    .into_dto()?;

    Ok(HttpResponse::Ok().json(event))
}

async fn delete_event(path: Path<(String, Uuid)>, role: Role, db: Data<Pool>) -> HttpResult {
    debug!(class_id = ?path.0, event_id = ?path.1, ?role, "delete event");

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

async fn get_timetable(class_id: Path<Uuid>, _role: Role, db: Data<Pool>) -> HttpResult {
    debug!(%class_id, ?_role, "get timetable");

    let timetable = block(move || actions::class::get_timetable(&db, *class_id))
        .await?
        .timetable;

    Ok(HttpResponse::Ok()
        .header("content-type", "application/json")
        .body(timetable))
}

async fn edit_timetable(
    class_id: Path<Uuid>,
    role: Role,
    db: Data<Pool>,
    table: Json<Timetable>,
) -> HttpResult {
    debug!(%class_id, ?role, ?table, "edit timetable");

    if !role.has_rights() {
        return Err(ServiceErr::NoAdminPermissions);
    }

    let table = table
        .into_inner()
        .iter_mut()
        .map(|day| {
            day.sort_unstable();
            day.clone() // todo oh
        })
        .collect::<Vec<_>>();

    let timetable = block(move || {
        actions::class::update_timetable(
            &db,
            models::Timetable {
                class: *class_id,
                timetable: serde_json::to_string(&table).map_err(|_| {
                    ServiceErr::InternalServerError("serialize-timetable".to_string())
                })?,
            },
        )
    })
    .await?
    .timetable;

    Ok(HttpResponse::Ok()
        .header("content-type", "application/json")
        .body(timetable))
}

async fn create_timetable(class_id: Path<Uuid>, role: Role, db: Data<Pool>) -> HttpResult {
    debug!(%class_id, ?role, "create timetable");

    if !role.has_rights() {
        return Err(ServiceErr::NoAdminPermissions);
    }

    let timetable = block(move || actions::class::create_timetable(&db, class_id.into_inner()))
        .await?
        .timetable;

    Ok(HttpResponse::Ok()
        .header("content-type", "application/json")
        .body(timetable))
}

async fn delete_timetable(class_id: Path<Uuid>, role: Role, db: Data<Pool>) -> HttpResult {
    debug!(%class_id, ?role, "delete timetable");

    if !role.has_rights() {
        return Err(ServiceErr::NoAdminPermissions);
    }

    let delete_count =
        block(move || actions::class::delete_timetable(&db, class_id.into_inner())).await?;

    Ok(match delete_count {
        0 => HttpResponse::NotFound().body("Timetable not found"),
        1 => HttpResponse::Ok().body("Deleted timetable."),
        _ => unreachable!(),
    })
}

async fn link_class_with_discord(
    class_id: Path<Uuid>,
    role: Role,
    db: Data<Pool>,
    id: Json<SingleSnowflake>,
) -> HttpResult {
    debug!(%class_id, ?role, ?id, "link class with discord");

    if *role != MemberRole::Owner {
        return Err(ServiceErr::BadRequest("no-owner"));
    }

    let snowflake = id.into_inner().snowflake;
    snowflake
        .parse::<u64>()
        .map_err(|_| ServiceErr::BadRequest("invalid-snowflake"))?;

    let class =
        block(move || actions::class::set_discord_id_class(&db, *class_id, Some(snowflake)))
            .await?
            .into_dto()?;

    Ok(HttpResponse::Ok().json(class))
}

async fn get_class_by_discord(
    class_id: Path<String>,
    claims: Claims,
    db: Data<Pool>,
) -> HttpResult {
    debug!(%class_id, uid = %claims.uid, "get class by discord");

    if !claims.uid.is_nil() {
        return Err(ServiceErr::Unauthorized("bot-only"));
    }

    let class = block(move || actions::class::get_class_by_discord(&db, &class_id))
        .await?
        .into_dto()?;

    Ok(HttpResponse::Ok().json(class))
}

async fn edit_guild_settings(claims: Claims, db: Data<Pool>, guild: Json<Guild>) -> HttpResult {
    debug!(?guild, uid = %claims.uid, "edit guild settings");

    if !claims.uid.is_nil() {
        return Err(ServiceErr::Unauthorized("bot-only"));
    }

    let guild = block(move || {
        actions::class::change_guild_settings(
            &db,
            NewGuild {
                id: &guild.id,
                notif_channel: guild.notif_channel.as_deref(),
                notif_ping_role: guild.notif_ping_role.as_deref(),
                notif_ping_everyone: guild.notif_ping_everyone,
            },
        )
    })
    .await?
    .into_dto()?;
    Ok(HttpResponse::Ok().json(guild))
}

async fn get_guild(guild_id: Path<String>, claims: Claims, db: Data<Pool>) -> HttpResult {
    debug!(?guild_id, uid = %claims.uid, "get guild");

    if !claims.uid.is_nil() {
        return Err(ServiceErr::Unauthorized("bot-only"));
    }

    let guild = block(move || actions::class::get_guild_settings(&db, &guild_id))
        .await?
        .into_dto()?;
    Ok(HttpResponse::Ok().json(guild))
}
