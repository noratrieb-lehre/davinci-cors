use crate::actions::{DbResult, Pool};
use crate::diesel::{QueryDsl, RunQueryDsl};
use crate::models::{Event, EventType, NewEvent};

use crate::schema::event_types::dsl::event_types;
use crate::schema::events::dsl::*;
use diesel::{insert_into, ExpressionMethods, SaveChangesDsl};
use uuid::Uuid;

pub fn get_events_by_class(db: &Pool, class_id: Uuid) -> DbResult<Vec<(Event, EventType)>> {
    let conn = db.get()?;

    let vec: Vec<(Event, EventType)> = events
        .filter(id.eq(class_id))
        .inner_join(event_types)
        .load(&conn)?;

    Ok(vec)
}

pub fn get_event_by_id(db: &Pool, event_id: Uuid) -> DbResult<Event> {
    let conn = db.get()?;

    Ok(events.find(event_id).get_result(&conn)?)
}

pub fn update_event(db: &Pool, new_event: NewEvent) -> DbResult<Event> {
    let conn = db.get()?;

    Ok(new_event.save_changes(&*conn)?)
}

pub fn insert_event(db: &Pool, new_event: NewEvent) -> DbResult<Event> {
    let conn = db.get()?;

    Ok(insert_into(events).values(&new_event).get_result(&conn)?)
}

//pub fn delete_event(db: &Pool, new_event: )
