use crate::actions::Pool;
use crate::diesel::{QueryDsl, RunQueryDsl};
use crate::models::{Class, Event, Guild, NewEvent};

use crate::error::ServiceResult;
use crate::schema::events::dsl::*;
use diesel::sql_types::{Nullable, Timestamp, VarChar};
use diesel::{
    delete, insert_into, BoolExpressionMethods, ExpressionMethods, JoinOnDsl, SaveChangesDsl,
};
use uuid::Uuid;

pub fn get_events_by_class(db: &Pool, class_id: Uuid) -> ServiceResult<Vec<Event>> {
    let conn = db.get()?;

    let vec: Vec<Event> = events.filter(class.eq(class_id)).load(&conn)?;

    Ok(vec)
}

pub fn get_events_by_class_filtered_after(
    db: &Pool,
    class_id: Uuid,
    after: chrono::NaiveDateTime,
) -> ServiceResult<Vec<Event>> {
    let conn = db.get()?;

    sql_function!(fn coalesce(a: Nullable<Timestamp>, b: Timestamp) -> Timestamp);

    let vec: Vec<Event> = events
        .filter(class.eq(class_id).and(coalesce(end, start).gt(after)))
        .load(&conn)?;

    Ok(vec)
}

pub fn get_events_by_class_filtered_before(
    db: &Pool,
    class_id: Uuid,
    before: chrono::NaiveDateTime,
) -> ServiceResult<Vec<Event>> {
    let conn = db.get()?;

    let vec: Vec<Event> = events
        .filter(class.eq(class_id).and(start.lt(before)))
        .load(&conn)?;

    Ok(vec)
}

pub fn get_events_by_class_filtered_both(
    db: &Pool,
    class_id: Uuid,
    before: chrono::NaiveDateTime,
    after: chrono::NaiveDateTime,
) -> ServiceResult<Vec<Event>> {
    let conn = db.get()?;

    sql_function!(fn coalesce(a: Nullable<Timestamp>, b: Timestamp) -> Timestamp);

    let vec: Vec<Event> = events
        .filter(
            class
                .eq(class_id)
                .and(start.lt(before).and(coalesce(end, start).gt(after))),
        )
        .load(&conn)?;

    Ok(vec)
}

pub fn get_event_by_id(db: &Pool, event_id: Uuid) -> ServiceResult<Event> {
    let conn = db.get()?;

    Ok(events.find(event_id).get_result(&conn)?)
}

pub fn update_event(db: &Pool, new_event: NewEvent) -> ServiceResult<Event> {
    let conn = db.get()?;

    Ok(new_event.save_changes(&*conn)?)
}

pub fn insert_event(db: &Pool, new_event: NewEvent) -> ServiceResult<Event> {
    let conn = db.get()?;

    Ok(insert_into(events).values(&new_event).get_result(&conn)?)
}

pub fn delete_event(db: &Pool, event_id: Uuid) -> ServiceResult<usize> {
    let conn = db.get()?;

    Ok(delete(events).filter(id.eq(event_id)).execute(&conn)?)
}

pub fn get_notifications(
    db: &Pool,
    since: chrono::NaiveDateTime,
) -> ServiceResult<(chrono::NaiveDateTime, Vec<(Event, (Class, Guild))>)> {
    use crate::schema::classes::dsl::{classes, discord_id};
    use crate::schema::guilds::dsl::{guilds, id as gid, notif_channel};

    let conn = db.get()?;

    let current_time = chrono::Utc::now().naive_utc();

    sql_function!(fn coalesce(a: Nullable<VarChar>, b: VarChar) -> VarChar);

    let notifications = events
        .inner_join(classes.inner_join(guilds.on(coalesce(discord_id, "").eq(gid))))
        .filter(
            notification
                .is_not_null()
                .and(notification.lt(current_time))
                .and(notification.gt(since))
                .and(notif_channel.is_not_null()),
        )
        .load(&conn)?;

    Ok((current_time, notifications))
}
