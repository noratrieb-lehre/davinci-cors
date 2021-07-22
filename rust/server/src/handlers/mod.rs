use crate::actions::{self, Pool};
use crate::error::ServiceErr;
use crate::handlers::auth::{create_normal_jwt, create_refresh_jwt, Claims};
use crate::models::conversion::IntoDto;
use actix_web::error::BlockingError;
use actix_web::web::Json;
use actix_web::{web, HttpResponse};
use diesel::result::DatabaseErrorKind;
use dto::{PostUser, User, UserPostResponse};
use jsonwebtoken::EncodingKey;

mod auth;
mod class;
mod extractors;

pub type HttpResult = Result<HttpResponse, ServiceErr>;

pub fn config(cfg: &mut web::ServiceConfig) {
    other_config(cfg);
    class::class_config(cfg);
    auth::auth_config(cfg);
}

pub fn other_config(cfg: &mut web::ServiceConfig) {
    cfg.route("/hugo", web::get().to(get_hugo)).service(
        web::scope("/users")
            .route("", web::post().to(create_user))
            .route("/me", web::get().to(get_own_user))
            .route("/me", web::put().to(edit_own_user))
            .route("/me", web::delete().to(delete_own_user)),
    );
}

async fn get_hugo() -> HttpResponse {
    HttpResponse::Ok().body("Hugo Boss")
}

async fn create_user(
    body: web::Json<PostUser>,
    db: web::Data<Pool>,
    key: web::Data<EncodingKey>,
) -> HttpResult {
    let user = match web::block(move || actions::user::insert_user(&db, &body)).await {
        Ok(user) => user,
        Err(BlockingError::Error(ServiceErr::DbActionFailed(
            diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _),
        ))) => Err(ServiceErr::Conflict("Email already exists".to_string()))?,
        other => other?,
    };

    let (token, expires) = create_normal_jwt(user.id, &key)?;
    let refresh_token = create_refresh_jwt(user.id, &key)?;

    Ok(HttpResponse::Created()
        .header("Token", token)
        .header("Refresh-Token", refresh_token)
        .json(UserPostResponse {
            id: user.id,
            email: user.email,
            description: user.description,
            expires,
        }))
}

async fn get_own_user(claims: Claims, db: web::Data<Pool>) -> HttpResult {
    let (mut user, classes) = web::block::<_, _, ServiceErr>(move || {
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

async fn edit_own_user(
    claims: Claims,
    db: web::Data<Pool>,
    mut new_user: Json<User>,
) -> HttpResult {
    new_user.id = claims.uid; // always update the own user
    let user = web::block(move || actions::user::update_user(&db, new_user.into_inner().into()))
        .await?
        .into_dto()?;

    Ok(HttpResponse::Ok().json(user))
}

async fn delete_own_user(claims: Claims, db: web::Data<Pool>) -> HttpResult {
    let amount = web::block(move || actions::user::delete_user(&db, claims.uid)).await?;

    Ok(match amount {
        0 => HttpResponse::NotFound().body("User not found"),
        1 => HttpResponse::Ok().body("Deleted user."),
        _ => unreachable!(),
    })
}
