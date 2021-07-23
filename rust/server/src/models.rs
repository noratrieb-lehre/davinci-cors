use crate::schema::*;
use diesel::{Identifiable, Insertable, Queryable};
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Insertable, Identifiable)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub description: String,
    pub discord_id: Option<String>,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: Uuid,
    pub email: &'a str,
    pub password: &'a str,
    pub description: &'a str,
    pub discord_id: Option<&'a str>,
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
    pub discord_id: Option<String>,
}

#[derive(Debug, Insertable, Queryable, Identifiable)]
#[table_name = "classes"]
pub struct NewClass<'a> {
    pub id: Uuid,
    pub owner: Uuid,
    pub name: &'a str,
    pub description: &'a str,
    pub discord_id: Option<&'a str>,
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
pub struct NewMember<'a> {
    pub user: Uuid,
    pub class: Uuid,
    pub display_name: &'a str,
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
    pub id: Uuid,
    pub class: Uuid,
    pub e_type: i32,
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
    use crate::models::{Class, Event, Member, MemberRole, User, PENDING};

    pub trait IntoDto<T> {
        fn into_dto(self) -> ServiceResult<T>;
    }

    impl IntoDto<dto::Class> for Class {
        fn into_dto(self) -> ServiceResult<dto::Class> {
            Ok(dto::Class {
                id: self.id,
                members: vec![],
                name: self.name,
                description: self.description,
            })
        }
    }

    impl IntoDto<dto::Class> for (Class, Vec<(Member, MemberRole)>) {
        fn into_dto(self) -> ServiceResult<dto::Class> {
            let (class, members) = self;

            let actual_members = members
                .into_iter()
                .filter(|(_, role)| role.id != PENDING)
                .map(IntoDto::into_dto)
                .collect::<Result<Vec<_>, _>>()?;

            Ok(dto::Class {
                id: class.id,
                members: actual_members,
                name: class.name,
                description: class.description,
            })
        }
    }

    impl IntoDto<dto::Member> for (Member, MemberRole) {
        fn into_dto(self) -> ServiceResult<dto::Member> {
            let (member, role) = self;
            Ok(dto::Member {
                user: member.user,
                display_name: member.display_name,
                role: role.into_dto()?,
            })
        }
    }

    impl IntoDto<dto::MemberRole> for MemberRole {
        fn into_dto(self) -> ServiceResult<dto::MemberRole> {
            Ok(match &*self.display {
                "owner" => dto::MemberRole::Owner,
                "admin" => dto::MemberRole::Admin,
                "member" => dto::MemberRole::Member,
                role => {
                    return Err(ServiceErr::IntoDTOError(format!(
                        "Invalid member role {}",
                        role
                    )))
                }
            })
        }
    }

    impl IntoDto<dto::MemberRole> for i32 {
        fn into_dto(self) -> ServiceResult<dto::MemberRole> {
            Ok(match self {
                0 => dto::MemberRole::Owner,
                1 => dto::MemberRole::Admin,
                2 => dto::MemberRole::Member,
                3 => dto::MemberRole::Pending,
                role => {
                    return Err(ServiceErr::IntoDTOError(format!(
                        "Invalid member role {}",
                        role
                    )))
                }
            })
        }
    }

    impl IntoDto<dto::EventType> for i32 {
        fn into_dto(self) -> ServiceResult<dto::EventType> {
            Ok(match self {
                1 => dto::EventType::Homework,
                2 => dto::EventType::Exam,
                3 => dto::EventType::Holidays,
                4 => dto::EventType::Other,
                role => {
                    return Err(ServiceErr::IntoDTOError(format!(
                        "Invalid member role {}",
                        role
                    )))
                }
            })
        }
    }

    impl IntoDto<dto::User> for User {
        fn into_dto(self) -> ServiceResult<dto::User> {
            Ok(dto::User {
                id: self.id,
                email: self.email,
                description: self.description,
                classes: None,
            })
        }
    }

    impl IntoDto<dto::User> for (User, Vec<dto::Class>) {
        fn into_dto(self) -> ServiceResult<dto::User> {
            let (user, classes) = self;
            Ok(dto::User {
                id: user.id,
                email: user.email,
                description: user.description,
                classes: Some(classes),
            })
        }
    }

    impl IntoDto<dto::Member> for Member {
        fn into_dto(self) -> ServiceResult<dto::Member> {
            Ok(dto::Member {
                user: self.user,
                display_name: self.display_name,
                role: self.role.into_dto()?,
            })
        }
    }

    impl IntoDto<dto::Event> for Event {
        fn into_dto(self) -> ServiceResult<dto::Event> {
            let end = self.end.map(|dt| dt.timestamp());
            let end = if let Some(0) = end { None } else { end };

            Ok(dto::Event {
                id: self.id,
                r#type: self.e_type.into_dto()?,
                name: self.name,
                start: self.start.timestamp(),
                end,
                description: self.description,
            })
        }
    }

    impl<T, Dto> IntoDto<Vec<Dto>> for Vec<T>
    where
        T: IntoDto<Dto>,
    {
        fn into_dto(self) -> ServiceResult<Vec<Dto>> {
            self.into_iter()
                .map(IntoDto::into_dto)
                .collect::<Result<Vec<Dto>, ServiceErr>>()
        }
    }

    ////// from dto

    impl From<dto::User> for User {
        fn from(user: dto::User) -> Self {
            Self {
                id: user.id,
                email: user.email,
                password: "".to_string(),
                description: user.description,
                discord_id: None,
            }
        }
    }

    pub fn member_role_dto_to_int(dto: &dto::MemberRole) -> i32 {
        *dto as i32
    }
}
