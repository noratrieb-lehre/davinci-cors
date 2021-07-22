use serde::{Deserialize, Serialize};

/// A UTC Unix timestamp in seconds
type Timestamp = i64;

/// A UTC seconds after 00:00
type DayTimestamp = u32;

/// A Unique User Id
type Uuid = uuid::Uuid;

/// A class event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub r#type: EventType,
    pub name: String,
    pub start: Timestamp,
    #[serde(default)]
    pub end: Timestamp,
    pub description: String,
}

/// The type of a class event
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventType {
    Homework = 1,
    Exam = 2,
    Holidays = 3,
    Other = 4,
}

/// A Class
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Class {
    #[serde(default)]
    pub id: Uuid,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<Member>,
    pub name: String,
    pub description: String,
}

/// A User
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(default)]
    pub id: Uuid,
    pub email: String,
    pub description: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classes: Option<Vec<Class>>,
}

/// The user for the `POST /users` route, with a password
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PostUser {
    #[serde(default)]
    pub id: Uuid,
    pub email: String,
    #[serde(default)]
    pub description: String,
    pub password: String,
}

/// A member (User in a class)
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub user: Uuid,
    pub display_name: String,
    pub role: MemberRole,
}

/// The role of a member
#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum MemberRole {
    Owner = 0,
    Admin = 1,
    Member = 2,
}

impl MemberRole {
    pub fn has_rights(&self) -> bool {
        matches!(self, MemberRole::Owner | MemberRole::Admin)
    }
}

/// The timetable of a class
pub type Timetable = [TimeTableDay; 7];

/// A day in the timetable of a class
pub type TimeTableDay = Vec<Lesson>;

/// A lesson in a timetable
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Lesson {
    pub subject: String,
    pub description: String,
    pub start: DayTimestamp,
    pub end: DayTimestamp,
}

/// Response of /token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshResponse {
    pub expires: i64,
}

/// Response of /token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub userid: Uuid,
    pub expires: i64,
}

/// Request body of /classes/{uuid}/requests/{uuid}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberAcceptDto {
    pub accept: bool,
}

/// Request body of /login
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

/// Response body of POST /users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPostResponse {
    pub id: Uuid,
    pub email: String,
    pub description: String,
    pub expires: Timestamp,
}
