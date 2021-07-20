use crate::actions::{DbResult, Pool};
use crate::diesel::{QueryDsl, RunQueryDsl};
use crate::models::{Class, Member, MemberRole, NewClass, Timetable};
use crate::schema::classes::dsl::*;
use diesel::{delete, insert_into, update, ExpressionMethods, SaveChangesDsl};
use uuid::Uuid;

pub fn insert_class(db: &Pool, new_class: NewClass) -> DbResult<Class> {
    let conn = db.get()?;

    Ok(insert_into(classes).values(&new_class).get_result(&conn)?)
}

type ClassMemberData = (Class, Vec<(Member, MemberRole)>);

pub fn get_class(db: &Pool, class_id: Uuid) -> DbResult<Option<ClassMemberData>> {
    use crate::schema::member_roles::dsl::member_roles;
    use crate::schema::members::dsl::{display_name, members, role};
    let conn = db.get()?;

    let vec: Vec<(Class, (Member, MemberRole))> = classes
        .filter(id.eq(class_id))
        .inner_join(members.inner_join(member_roles))
        .order_by((role, display_name))
        .load(&conn)?;

    Ok(map_class_join_members(vec))
}

pub fn update_class(db: &Pool, new_class: NewClass) -> DbResult<Class> {
    let conn = db.get()?;

    Ok(new_class.save_changes(&*conn)?)
}

pub fn delete_class(db: &Pool, class_id: Uuid) -> DbResult<usize> {
    let conn = db.get()?;

    Ok(delete(classes).filter(id.eq(class_id)).execute(&conn)?)
}

pub fn get_timetable(db: &Pool, class_id: Uuid) -> DbResult<Timetable> {
    use crate::schema::timetables::dsl::*;
    let conn = db.get()?;

    Ok(timetables.find(class_id).get_result(&conn)?)
}

pub fn create_timetable(db: &Pool, class_id: Uuid) -> DbResult<Timetable> {
    use crate::schema::timetables::dsl::*;
    let conn = db.get()?;

    Ok(insert_into(timetables)
        .values(class.eq(class_id))
        .get_result(&conn)?)
}

pub fn update_timetable(db: &Pool, new_timetable: Timetable) -> DbResult<Timetable> {
    use crate::schema::timetables::dsl::*;
    let conn = db.get()?;

    Ok(update(timetables)
        .filter(class.eq(new_timetable.class))
        .set(timetable.eq(new_timetable.timetable))
        .get_result(&conn)?)
}

pub fn delete_timetable(db: &Pool, class_id: Uuid) -> DbResult<usize> {
    use crate::schema::timetables::dsl::*;
    let conn = db.get()?;

    Ok(delete(timetables)
        .filter(class.eq(class_id))
        .execute(&conn)?)
}

fn map_class_join_members(vec: Vec<(Class, (Member, MemberRole))>) -> Option<ClassMemberData> {
    match vec
        .into_iter()
        .fold((None, vec![]), |(_, mut vec), (class, member)| {
            vec.push(member);
            (Some(class), vec)
        }) {
        (Some(class), vec) => Some((class, vec)),
        (None, _) => None,
    }
}
