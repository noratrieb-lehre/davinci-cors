use crate::schema::*;
use diesel::{Insertable, Queryable};
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

#[derive(Debug, Clone, Queryable)]
pub struct MemberRole {
    pub id: i32,
    pub display: String,
}

#[derive(Debug, Clone, Queryable)]
pub struct Class {
    pub id: Uuid,
    pub owner: Uuid,
    pub name: String,
    pub description: String,
    pub timetable: String, // todo note: this should be JSON
}

#[derive(Debug, Insertable, Queryable)]
#[table_name = "class"]
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
