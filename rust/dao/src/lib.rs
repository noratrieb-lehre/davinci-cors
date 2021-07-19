use serde::{Deserialize, Serialize};

/// A UTC Unix timestamp in seconds
type Timestamp = u64;

/// A UTC seconds after 00:00
type DayTimestamp = u32;

/// A Unique User Id
type UUID = String;

/// A class event
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EventDao {
    r#type: String,
    name: String,
    start: Timestamp,
    #[serde(default)]
    end: Timestamp,
    description: String,
}

/// The type of a class event
/// unused until i find out how to use it
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum EventType {
    Homework,
    Exam,
    Holidays,
    Other(String),
}

/// A Class
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
struct Class {
    id: UUID,
    owner: User,
    members: Vec<Member>,
    name: String,
    description: String,
}

/// A User
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
struct User {
    id: UUID,
    email: String,
    description: String,
    #[serde(default)]
    classes: Vec<Class>,
}

/// A member (User in a class)
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Member {
    user: UUID,
    class: UUID,
    display_name: String,
    role: MemberRole,
}

/// The role of a member
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum MemberRole {
    Owner,
    Admin,
    Member,
}

/// The timetable of a class
pub type Timetable = [TimeTableDay; 7];

/// A day in the timetable of a class
pub type TimeTableDay = Vec<Lesson>;

/// A lesson in a timetable
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Lesson {
    subject: String,
    description: String,
    start: DayTimestamp,
    end: DayTimestamp,
}

/// Response of /token
#[derive(Debug, Clone, Serialize)]
pub struct RefreshResponse {
    pub expires: i64,
}
