mod class;

pub type DbResult<T> = Result<T, crate::error::ServiceErr>;

mod user {
    use crate::actions::DbResult;
    use crate::models::User;
    use crate::Pool;
    //use diesel::{QueryDsl, RunQueryDsl};
    use uuid::Uuid;

    pub fn get_user_by_id(db: &Pool, user_id: Uuid) -> DbResult<User> {
        //use crate::schema::users::dsl::*;
        let conn = db.get()?;
        //users.find(user_id).get_result::<User>(&conn)?
        todo!()
    }
}
