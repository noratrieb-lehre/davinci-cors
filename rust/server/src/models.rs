use crate::schema::*;
use diesel::{Identifiable, Insertable, Queryable};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Insertable, Identifiable)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub description: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a Uuid,
    pub email: &'a str,
    pub password: &'a str,
    pub description: &'a str,
}

#[derive(Debug, Clone, Queryable, Identifiable)]
pub struct MemberRole {
    pub id: i32,
    pub display: String,
}

#[derive(Debug, Clone, Queryable, Identifiable)]
#[table_name = "classes"]
pub struct Class {
    pub id: Uuid,
    pub owner: Uuid,
    pub name: String,
    pub description: String,
    pub timetable: String, // todo note: this should be JSON
}

#[derive(Debug, Insertable, Queryable, Identifiable, AsChangeset)]
#[table_name = "classes"]
pub struct NewClass<'a> {
    pub id: &'a Uuid,
    pub owner: &'a Uuid,
    pub name: &'a str,
    pub description: &'a str,
    pub timetable: &'a str,
}

#[derive(Debug, Clone, Queryable)]
pub struct Member {
    pub user: Uuid,
    pub class: Uuid,
    pub display_name: String,
    pub role: i32,
}

#[derive(Debug, Clone, Queryable)]
pub struct Event {
    pub id: Uuid,
    pub class: Uuid,
    pub e_type: i32,
    pub name: String,
    pub start: chrono::NaiveDateTime,
    pub end: Option<chrono::NaiveDateTime>,
    pub description: String,
}

#[derive(Debug, Insertable, Queryable, Identifiable, AsChangeset)]
#[table_name = "events"]
pub struct NewEvent<'a> {
    pub id: &'a Uuid,
    pub class: &'a Uuid,
    pub e_type: &'a i32,
    pub name: &'a str,
    pub start: &'a chrono::NaiveDateTime,
    pub end: &'a chrono::NaiveDateTime,
    pub description: &'a str,
}

#[derive(Debug, Clone, Queryable)]
pub struct EventType {
    pub id: i32,
    pub display: String,
}
