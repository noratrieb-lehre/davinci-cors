use crate::actions::{DbResult, Pool};
use crate::diesel::{QueryDsl, RunQueryDsl};
use crate::models::{Class, Member, MemberRole, NewClass};
use crate::schema::classes::dsl::*;
use diesel::{delete, insert_into, ExpressionMethods, SaveChangesDsl};
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

fn update_class(db: &Pool, new_class: NewClass) -> DbResult<Class> {
    let conn = db.get()?;

    Ok(new_class.save_changes(&*conn)?)
}

fn delete_class(db: &Pool, class_id: Uuid) -> DbResult<()> {
    let conn = db.get()?;

    delete(classes).filter(id.eq(class_id)).execute(&conn)?;
    Ok(())
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
