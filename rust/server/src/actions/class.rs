use crate::actions::DbResult;
use crate::diesel::{QueryDsl, RunQueryDsl};
use crate::models::{Class, Member, MemberRole, NewClass, User};
use crate::Pool;
use diesel::{delete, insert_into, update, BoolExpressionMethods, ExpressionMethods};
use uuid::Uuid;

pub fn insert_class(db: &Pool, new_class: NewClass) -> DbResult<Class> {
    use crate::schema::class::dsl::*;
    let conn = db.get()?;

    Ok(insert_into(class).values(&new_class).get_result(&conn)?)
}

type ClassMemberData = (Class, Vec<(Member, MemberRole)>);

pub fn get_class(db: &Pool, class_id: Uuid) -> DbResult<Option<ClassMemberData>> {
    use crate::schema::class::dsl::{class, id};
    use crate::schema::member::dsl::member;
    use crate::schema::member_role::dsl::member_role;
    let conn = db.get()?;

    let vec: Vec<(Class, (Member, MemberRole))> = class
        .filter(id.eq(class_id))
        .inner_join(member.inner_join(member_role))
        .load(&conn)?;

    Ok(map_class_join_members(vec))
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
