use super::Pool;
use crate::diesel::{QueryDsl, RunQueryDsl};
use crate::error::{ServiceErr, ServiceResult};
use crate::models::{NewUser, User};
use crate::schema::users::dsl::*;
use diesel::sql_types::{Integer, Text};
use diesel::{delete, insert_into, update, BoolExpressionMethods, ExpressionMethods};
use uuid::Uuid;

sql_function!(fn crypt(pwd: Text, salt: Text) -> Text);
sql_function!(fn gen_salt(kind: Text, number: Integer) -> Text);

pub fn get_user_by_id(db: &Pool, user_id: Uuid) -> ServiceResult<User> {
    let conn = db.get()?;
    Ok(users.find(user_id).get_result::<User>(&conn)?)
}

pub fn validate_user_password(
    db: &Pool,
    u_email: &str,
    u_password: &str,
) -> ServiceResult<Option<User>> {
    let conn = db.get()?;

    let found_user: Vec<User> = users
        .filter(
            email
                .eq(u_email)
                .and(password.eq(crypt(u_password, password))),
        )
        .get_results::<User>(&conn)?;

    Ok(found_user.into_iter().next())
}

pub fn insert_user(db: &Pool, user: NewUser) -> ServiceResult<User> {
    let conn = db.get()?;

    Ok(insert_into(users)
        .values((
            id.eq(user.id),
            email.eq(user.email),
            password.eq(crypt(user.password, gen_salt("bf", 8))),
            description.eq(user.description),
        ))
        .get_result(&conn)?)
}

pub fn delete_user(db: &Pool, user_id: Uuid) -> ServiceResult<usize> {
    let conn = db.get()?;

    Ok(delete(users).filter(id.eq(user_id)).execute(&conn)?)
}

pub fn update_user(db: &Pool, user: User) -> ServiceResult<User> {
    let conn = db.get()?;

    Ok(update(users.filter(id.eq(user.id)))
        .set((description.eq(user.description), (email.eq(user.email))))
        .get_result(&conn)?)
}

pub fn increment_token_version(db: &Pool, uid: Uuid) -> ServiceResult<User> {
    let conn = db.get()?;

    Ok(update(users.filter(id.eq(uid)))
        .set(token_version.eq(token_version + 1))
        .get_result(&conn)?)
}

pub fn change_user_password(db: &Pool, user: User) -> ServiceResult<User> {
    let conn = db.get()?;

    Ok(update(users.filter(id.eq(user.id)))
        .set(password.eq(crypt(user.password, gen_salt("bf", 8))))
        .get_result(&conn)?)
}

pub fn set_discord_id_user(db: &Pool, user_id: Uuid, d_id: Option<&str>) -> ServiceResult<User> {
    let conn = db.get()?;

    Ok(update(users)
        .filter(id.eq(user_id))
        .set(discord_id.eq(d_id))
        .get_result(&conn)?)
}

pub fn get_user_by_discord(db: &Pool, user_id: &str) -> ServiceResult<User> {
    let conn = db.get()?;

    users
        .filter(discord_id.eq(user_id))
        .load(&conn)?
        .into_iter()
        .next()
        .ok_or(ServiceErr::NotFound)
}
