use crate::actions::Pool;
use crate::diesel::{QueryDsl, RunQueryDsl};
use crate::models::{Event, NewEvent};

use crate::error::ServiceResult;
use crate::schema::events::dsl::*;
use diesel::{delete, insert_into, ExpressionMethods, SaveChangesDsl};
use uuid::Uuid;

pub fn get_events_by_class(db: &Pool, class_id: Uuid) -> ServiceResult<Vec<Event>> {
    let conn = db.get()?;

    let vec: Vec<Event> = events.filter(id.eq(class_id)).load(&conn)?;

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
