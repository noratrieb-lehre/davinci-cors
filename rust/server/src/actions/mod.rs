mod class;

pub type DbResult<T> = Result<T, crate::error::ServiceErr>;

mod user {
    use crate::actions::DbResult;
    use crate::diesel::{QueryDsl, RunQueryDsl};
    use crate::models::{NewUser, User};
    use crate::Pool;
    use diesel::sql_types::{Integer, Text};
    use diesel::{delete, insert_into, update, BoolExpressionMethods, ExpressionMethods};
    use uuid::Uuid;

    sql_function!(fn crypt(password: Text, salt: Text) -> Text);
    sql_function!(fn gen_salt(kind: Text, number: Integer) -> Text);

    pub fn get_user_by_id(db: &Pool, user_id: Uuid) -> DbResult<User> {
        use crate::schema::users::dsl::*;
        let conn = db.get()?;
        Ok(users.find(user_id).get_result::<User>(&conn)?)
    }

    pub fn validate_user_password(db: &Pool, u_email: &str, u_password: &str) -> DbResult<bool> {
        use crate::schema::users::dsl::*;
        let conn = db.get()?;

        let user: Vec<String> = users
            .select(email)
            .filter(
                email
                    .eq(u_email)
                    .and(password.eq(crypt(u_password, password))),
            )
            .get_results::<String>(&conn)?;

        Ok(user.len() == 1)
    }

    pub fn insert_user(db: &Pool, new_user: NewUser) -> DbResult<User> {
        use crate::schema::users::dsl::*;
        let conn = db.get()?;

        let new_uuid = uuid::Uuid::new_v4();

        Ok(insert_into(users)
            .values((
                id.eq(new_uuid),
                email.eq(new_user.email),
                password.eq(crypt(new_user.password, gen_salt("bf", 8))),
                description.eq(new_user.description),
            ))
            .get_result(&conn)?)
    }

    pub fn delete_user(db: &Pool, user_id: Uuid) -> DbResult<()> {
        use crate::schema::users::dsl::*;
        let conn = db.get()?;

        delete(users).filter(id.eq(user_id)).execute(&conn)?;
        Ok(())
    }

    pub fn update_user_description(db: &Pool, user: User) -> DbResult<User> {
        use crate::schema::users::dsl::*;
        let conn = db.get()?;

        Ok(update(users.filter(id.eq(user.id)))
            .set((description.eq(user.description), (email.eq(user.email))))
            .get_result(&conn)?)
    }

    pub fn change_user_password(db: &Pool, user: User) -> DbResult<User> {
        use crate::schema::users::dsl::*;
        let conn = db.get()?;

        Ok(update(users.filter(email.eq(user.email)))
            .set(password.eq(crypt(user.password, gen_salt("bf", 8))))
            .get_result(&conn)?)
    }
}
