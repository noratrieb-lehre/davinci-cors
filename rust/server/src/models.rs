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
}

#[derive(Debug, Insertable, Queryable, Identifiable, AsChangeset)]
#[table_name = "classes"]
pub struct NewClass {
    pub id: Uuid,
    pub owner: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Queryable)]
pub struct Member {
    pub user: Uuid,
    pub class: Uuid,
    pub display_name: String,
    pub role: i32,
}

#[derive(Debug, Clone, Insertable, Identifiable, AsChangeset)]
#[table_name = "members"]
#[primary_key(user, class)]
pub struct NewMember {
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

#[derive(Debug, Clone, Queryable)]
pub struct Timetable {
    pub class: Uuid,
    pub timetable: String,
}

#[derive(Debug, Insertable)]
#[table_name = "timetables"]
pub struct NewTimetable<'a> {
    pub class: &'a Uuid,
    pub timetable: &'a str,
}

pub const PENDING: i32 = 3;

pub mod conversion {
    use crate::error::{ServiceErr, ServiceResult};
    use crate::models::{Class, Member, MemberRole, User, PENDING};
    use std::convert::TryFrom;

    pub trait IntoDao<T> {
        fn into_dao(self) -> ServiceResult<T>;
    }

    impl IntoDao<dao::Class> for Class {
        fn into_dao(self) -> ServiceResult<dao::Class> {
            Ok(dao::Class {
                id: self.id,
                members: vec![],
                name: self.name,
                description: self.description,
            })
        }
    }

    impl IntoDao<dao::Class> for (Class, Vec<(Member, MemberRole)>) {
        fn into_dao(self) -> ServiceResult<dao::Class> {
            let (class, members) = self;

            let actual_members = members
                .into_iter()
                .filter(|(_, role)| role.id != PENDING)
                .map(IntoDao::into_dao)
                .collect::<Result<Vec<_>, _>>()?;

            Ok(dao::Class {
                id: class.id,
                members: actual_members,
                name: class.name,
                description: class.description,
            })
        }
    }

    impl IntoDao<dao::Member> for (Member, MemberRole) {
        fn into_dao(self) -> ServiceResult<dao::Member> {
            let (member, role) = self;
            Ok(dao::Member {
                user: member.user,
                display_name: member.display_name,
                role: role.into_dao()?,
            })
        }
    }

    impl IntoDao<dao::MemberRole> for MemberRole {
        fn into_dao(self) -> ServiceResult<dao::MemberRole> {
            Ok(match &*self.display {
                "owner" => dao::MemberRole::Owner,
                "admin" => dao::MemberRole::Admin,
                "member" => dao::MemberRole::Member,
                role => Err(ServiceErr::InvalidDao(format!(
                    "Invalid member role {}",
                    role
                )))?,
            })
        }
    }

    impl IntoDao<dao::MemberRole> for i32 {
        fn into_dao(self) -> ServiceResult<dao::MemberRole> {
            Ok(match self {
                0 => dao::MemberRole::Owner,
                1 => dao::MemberRole::Admin,
                2 => dao::MemberRole::Member,
                role => Err(ServiceErr::InvalidDao(format!(
                    "Invalid member role {}",
                    role
                )))?,
            })
        }
    }

    impl IntoDao<dao::User> for User {
        fn into_dao(self) -> ServiceResult<dao::User> {
            Ok(dao::User {
                id: self.id,
                email: self.email,
                description: self.description,
                classes: None,
            })
        }
    }

    impl IntoDao<dao::User> for (User, Vec<dao::Class>) {
        fn into_dao(self) -> ServiceResult<dao::User> {
            let (user, classes) = self;
            Ok(dao::User {
                id: user.id,
                email: user.email,
                description: user.description,
                classes: Some(classes),
            })
        }
    }

    impl IntoDao<dao::Member> for Member {
        fn into_dao(self) -> ServiceResult<dao::Member> {
            Ok(dao::Member {
                user: self.user,
                display_name: self.display_name,
                role: self.role.into_dao()?,
            })
        }
    }

    ////// from dao

    impl From<dao::User> for User {
        fn from(user: dao::User) -> Self {
            Self {
                id: user.id,
                email: user.email,
                password: "".to_string(),
                description: user.description,
            }
        }
    }

    ////// new _

    impl TryFrom<dao::Class> for Class {
        type Error = ServiceErr;

        fn try_from(class: dao::Class) -> Result<Self, Self::Error> {
            Ok(Self {
                id: class.id,
                owner: class
                    .members
                    .into_iter()
                    .find(|member| member.role == dao::MemberRole::Owner)
                    .ok_or(ServiceErr::BadRequest("No Owner provided".to_string()))?
                    .user,
                name: class.name,
                description: class.description,
            })
        }
    }
}
