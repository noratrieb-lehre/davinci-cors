use crate::actions::{self, Pool};
use crate::error::ServiceErr;
use crate::handlers::auth::Claims;
use crate::models::conversion::IntoDto;
use actix_web::web::ServiceConfig;
use actix_web::web::{block, delete, get, post, put, scope, Data, Json, Path, Query};
use actix_web::HttpResponse;
use dto::{NotificationQueryParams, NotificationRes, SingleSnowflake, User};
use tracing::debug;

mod auth;
mod class;
mod extractors;

pub type HttpResult = Result<HttpResponse, ServiceErr>;

pub fn config(cfg: &mut ServiceConfig) {
    other_config(cfg);
    class::class_config(cfg);
    auth::auth_config(cfg);
}

pub fn other_config(cfg: &mut ServiceConfig) {
    cfg.route("/hugo", get().to(get_hugo))
        .route("/bot/notifications", get().to(get_notifications))
        .service(
            scope("/users")
                .route("/me", get().to(get_own_user))
                .route("/me", put().to(edit_own_user))
                .route("/me", delete().to(delete_own_user))
                .route("/me/link", post().to(link_user_with_discord))
                .route("/discord/{snowflake}", get().to(get_user_by_discord)),
        );
}

async fn get_hugo() -> HttpResponse {
    debug!("Someone got Hugo!");

    HttpResponse::Ok().body("Hugo Boss")
}

async fn get_own_user(claims: Claims, db: Data<Pool>) -> HttpResult {
    debug!(uid = %claims.uid, "get own user");

    let (mut user, classes) = block::<_, _, ServiceErr>(move || {
        let user = actions::user::get_user_by_id(&db, claims.uid)?;
        let classes = actions::class::get_classes_by_user(&db, claims.uid)?;

        Ok((
            user.into_dto()?,
            classes
                .into_iter()
                .map(IntoDto::into_dto)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    })
    .await?;

    user.classes = Some(classes);

    Ok(HttpResponse::Ok().json(user))
}

async fn edit_own_user(claims: Claims, db: Data<Pool>, mut new_user: Json<User>) -> HttpResult {
    debug!(uid = %claims.uid, ?new_user, "edit own user");

    new_user.id = claims.uid; // always update the own user
    let user = block(move || actions::user::update_user(&db, new_user.into_inner().into()))
        .await?
        .into_dto()?;

    Ok(HttpResponse::Ok().json(user))
}

async fn delete_own_user(claims: Claims, db: Data<Pool>) -> HttpResult {
    debug!(uid = %claims.uid, "delete own user 😔 rip");

    let amount = block(move || actions::user::delete_user(&db, claims.uid)).await?;

    Ok(match amount {
        0 => HttpResponse::NotFound().body("User not found"),
        1 => HttpResponse::Ok().body("Deleted user."),
        _ => unreachable!(),
    })
}

async fn link_user_with_discord(
    claims: Claims,
    db: Data<Pool>,
    id: Json<SingleSnowflake>,
) -> HttpResult {
    debug!(uid = %claims.uid, ?id, "link own user with discord");

    let snowflake = id.into_inner().snowflake;
    snowflake
        .parse::<u64>()
        .map_err(|_| ServiceErr::BadRequest("invalid-snowflake"))?;

    let user = block(move || actions::user::set_discord_id_user(&db, claims.uid, Some(&snowflake)))
        .await?
        .into_dto()?;

    Ok(HttpResponse::Ok().json(user))
}

async fn get_user_by_discord(user_id: Path<String>, db: Data<Pool>, claims: Claims) -> HttpResult {
    debug!(uid = %claims.uid, ?user_id, "get user by discord");

    if !claims.uid.is_nil() {
        return Err(ServiceErr::Unauthorized("bot-only"));
    }

    let user = block(move || actions::user::get_user_by_discord(&db, &user_id))
        .await?
        .into_dto()?;

    Ok(HttpResponse::Ok().json(user))
}

async fn get_notifications(
    params: Query<NotificationQueryParams>,
    db: Data<Pool>,
    claims: Claims,
) -> HttpResult {
    debug!(?params, "Called get notifications");
    if !claims.uid.is_nil() {
        return Err(ServiceErr::Unauthorized("bot-only"));
    }

    let (time, notifications) = block(move || {
        actions::event::get_notifications(
            &db,
            chrono::NaiveDateTime::from_timestamp(params.since / 1000, 0),
        )
    })
    .await?;

    let notifications = notifications.into_dto()?;

    Ok(HttpResponse::Ok().json(NotificationRes {
        notifications,
        time: time.timestamp_millis(),
    }))
}
