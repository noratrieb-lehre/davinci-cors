use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

pub mod class;
pub mod event;
pub mod user;

type Connection = ConnectionManager<PgConnection>;
pub type Pool = r2d2::Pool<Connection>;

#[cfg(test)]
mod test {
    use diesel::r2d2::ConnectionManager;
    use diesel::PgConnection;

    use super::class::*;
    use super::user::*;
    use crate::actions::event::{
        get_events_by_class, get_events_by_class_filtered_after,
        get_events_by_class_filtered_before, get_events_by_class_filtered_both, insert_event,
    };
    use crate::actions::Pool;
    use crate::models;
    use crate::models::{Class, NewClass, NewEvent, NewMember, NewUser, User};
    use chrono::NaiveDateTime;
    use dto::{Lesson, Timetable};

    fn get_pool() -> Pool {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    }

    #[test]
    fn get_all_class_members() {
        let db = get_pool();

        let user = insert_user(
            &db,
            NewUser {
                id: uuid::Uuid::new_v4(),
                email: "TEST_test",
                password: "xxxxxxsecretxxxxxx",
                description: "test",
                discord_id: None,
            },
        )
        .unwrap();

        assert_eq!(user.email, "TEST_test");
        assert_eq!(user.description, "test");
        assert_eq!(user.discord_id, None);

        let user_discord = set_discord_id_user(&db, user.id, Some("hallo")).unwrap();
        assert_eq!(user_discord.discord_id, Some("hallo".to_string()));
        assert_eq!(user_discord.id, user.id);

        let class = insert_class(
            &db,
            NewClass {
                id: uuid::Uuid::new_v4(),
                owner: user.id,
                name: "testklasse",
                description: "",
                discord_id: Some("4387208542528543"),
            },
        )
        .unwrap();

        assert_eq!(class.owner, user.id);
        assert_eq!(&*class.name, "testklasse");
        assert_eq!(class.description, "");
        assert_eq!(class.discord_id, Some("4387208542528543".to_string()));

        let class_by_discord = get_class_by_discord(&db, "4387208542528543").unwrap();
        assert_eq!(class_by_discord.id, class.id);

        let owner_member = create_member(
            &db,
            NewMember {
                user: user.id,
                class: class.id,
                display_name: "member",
                role: 0,
            },
        )
        .unwrap();

        assert_eq!(owner_member.class, class.id);
        assert_eq!(owner_member.user, user.id);
        assert_eq!(&*owner_member.display_name, "member");
        assert_eq!(owner_member.role, 0);

        let (full_class, members) = get_class(&db, class.id).unwrap().unwrap();

        assert_eq!(full_class.id, class.id);
        assert_eq!(full_class.name, class.name);
        assert_eq!(full_class.description, class.description);
        assert_eq!(members.len(), 1);
        let (member, role) = members.into_iter().next().unwrap();
        assert_eq!(member.user, user.id);
        assert_eq!(member.role, role.id);
        assert_eq!(&*role.display, "owner");

        delete_member(&db, user.id, class.id).unwrap();
        delete_class(&db, class.id).unwrap();
        delete_user(&db, user.id).unwrap();
    }

    #[test]
    fn timetables() {
        let db = get_pool();

        let (user, class) = insert_class_user(&db);

        let timetable = create_timetable(&db, class.id).unwrap();
        assert_eq!(&*timetable.timetable, "[[],[],[],[],[],[],[]]");

        let new_timetable: Timetable = [
            vec![Lesson {
                subject: "subj".to_string(),
                description: "".to_string(),
                start: 100,
                end: 200,
            }],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ];
        let new_timetable_json = serde_json::to_string(&new_timetable).unwrap();

        let updated_table = update_timetable(
            &db,
            models::Timetable {
                class: class.id,
                timetable: new_timetable_json,
            },
        )
        .unwrap();

        assert_eq!(updated_table.class, class.id);
        assert_eq!(
            serde_json::from_str::<Timetable>(&updated_table.timetable).unwrap(),
            new_timetable
        );

        delete_class(&db, class.id).unwrap();
        delete_user(&db, user.id).unwrap();
    }

    #[test]
    fn user_password() {
        let db = get_pool();

        let user = insert_user(
            &db,
            NewUser {
                id: uuid::Uuid::new_v4(),
                email: "test_password",
                password: "pass_wort",
                description: "test",
                discord_id: None,
            },
        )
        .unwrap();

        assert_eq!(user.email, "test_password");

        let wrong_password = validate_user_password(&db, "test_password", "pass_wrt").unwrap();
        assert!(matches!(wrong_password, None));

        let wrong_email = validate_user_password(&db, "test_passwrd", "pass_wort").unwrap();
        assert!(matches!(wrong_email, None));

        let correct = validate_user_password(&db, "test_password", "pass_wort").unwrap();
        assert!(matches!(correct, Some(_)));

        let validated_user = correct.unwrap();
        assert_eq!(validated_user.id, user.id);

        delete_user(&db, user.id).unwrap();
    }

    #[test]
    fn event_filter() {
        let db = get_pool();

        let (owner, class) = insert_class_user(&db);

        let events = vec![
            insert_event(
                &db,
                NewEvent {
                    id: uuid::Uuid::new_v4(),
                    class: class.id,
                    e_type: 1,
                    name: "event1000",
                    start: &NaiveDateTime::from_timestamp(1000, 0),
                    end: Some(&NaiveDateTime::from_timestamp(2000, 0)),
                    description: "event",
                    notification: None,
                },
            )
            .unwrap(),
            insert_event(
                &db,
                NewEvent {
                    id: uuid::Uuid::new_v4(),
                    class: class.id,
                    e_type: 1,
                    name: "event1000",
                    start: &NaiveDateTime::from_timestamp(2000, 0),
                    end: None,
                    description: "event",
                    notification: None,
                },
            )
            .unwrap(),
            insert_event(
                &db,
                NewEvent {
                    id: uuid::Uuid::new_v4(),
                    class: class.id,
                    e_type: 1,
                    name: "event1000",
                    start: &NaiveDateTime::from_timestamp(0, 0),
                    end: Some(&NaiveDateTime::from_timestamp(10000, 0)),
                    description: "event",
                    notification: None,
                },
            )
            .unwrap(),
        ];
        assert_eq!(events.len(), 3);

        let events = get_events_by_class(&db, class.id).unwrap();
        assert_eq!(events.len(), 3);

        let events = get_events_by_class_filtered_after(
            &db,
            class.id,
            chrono::NaiveDateTime::from_timestamp(0, 0),
        )
        .unwrap();
        assert_eq!(events.len(), 3);

        let events = get_events_by_class_filtered_before(
            &db,
            class.id,
            chrono::NaiveDateTime::from_timestamp(0, 0),
        )
        .unwrap();
        assert_eq!(events.len(), 0);

        let events = get_events_by_class_filtered_after(
            &db,
            class.id,
            chrono::NaiveDateTime::from_timestamp(1, 0),
        )
        .unwrap();
        assert_eq!(events.len(), 3);

        let events = get_events_by_class_filtered_after(
            &db,
            class.id,
            chrono::NaiveDateTime::from_timestamp(1000000, 0),
        )
        .unwrap();
        assert_eq!(events.len(), 0);

        let events = get_events_by_class_filtered_both(
            &db,
            class.id,
            NaiveDateTime::from_timestamp(1500, 0),
            chrono::NaiveDateTime::from_timestamp(500, 0),
        )
        .unwrap();
        assert_eq!(events.len(), 2);

        let events = get_events_by_class_filtered_both(
            &db,
            class.id,
            NaiveDateTime::from_timestamp(2001, 0),
            chrono::NaiveDateTime::from_timestamp(1999, 0),
        )
        .unwrap();
        assert_eq!(events.len(), 3);

        delete_class(&db, class.id).unwrap();
        delete_user(&db, owner.id).unwrap();
    }

    fn insert_class_user(db: &Pool) -> (User, Class) {
        let user = insert_user(
            &db,
            NewUser {
                id: uuid::Uuid::new_v4(),
                email: &uuid::Uuid::new_v4().to_string(),
                password: "xxxxxxsecretxxxxxx",
                description: "test",
                discord_id: None,
            },
        )
        .unwrap();

        let class = insert_class(
            &db,
            NewClass {
                id: uuid::Uuid::new_v4(),
                owner: user.id,
                name: "testklasse",
                description: "",
                discord_id: None,
            },
        )
        .unwrap();
        (user, class)
    }
}
