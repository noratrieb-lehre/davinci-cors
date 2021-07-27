use crate::actions::Pool;
use crate::diesel::{QueryDsl, RunQueryDsl};
use crate::models::{Event, FromNotificationQuery, NewEvent};

use crate::error::ServiceResult;
use crate::schema::events::dsl::*;
use diesel::sql_types::{Nullable, Timestamp};
use diesel::{
    delete, insert_into, sql_query, BoolExpressionMethods, ExpressionMethods, SaveChangesDsl,
};
use uuid::Uuid;

sql_function!(fn coalesce(a: Nullable<Timestamp>, b: Timestamp) -> Timestamp);

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
) -> ServiceResult<(chrono::NaiveDateTime, Vec<FromNotificationQuery>)> {
    let conn = db.get()?;

    let current_time = chrono::Utc::now().naive_utc();

    // See the comment in the sql file
    let notifications = sql_query(include_str!("get_notifications.sql"))
        .bind::<Timestamp, _>(current_time)
        .bind::<Timestamp, _>(since)
        .load(&conn)?;

    Ok((current_time, notifications))
}
