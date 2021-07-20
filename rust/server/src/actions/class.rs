use crate::actions::DbResult;
use crate::models::{Class, NewClass};
use crate::Pool;
use diesel::{insert_into, RunQueryDsl};

pub fn insert_class(db: &Pool, new_class: NewClass) -> DbResult<()> {
    use crate::schema::class::dsl::*;
    let conn = db.get()?;

    insert_into(class).values(&new_class).execute(&conn)?;
    Ok(())
}
