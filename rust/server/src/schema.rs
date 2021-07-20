table! {
    classes (id) {
        id -> Uuid,
        owner -> Uuid,
        name -> Varchar,
        description -> Varchar,
    }
}

table! {
    event_types (id) {
        id -> Int4,
        display -> Varchar,
    }
}

table! {
    events (id) {
        id -> Uuid,
        class -> Uuid,
        e_type -> Int4,
        name -> Varchar,
        start -> Timestamp,
        end -> Nullable<Timestamp>,
        description -> Varchar,
    }
}

table! {
    member_roles (id) {
        id -> Int4,
        display -> Varchar,
    }
}

table! {
    members (user, class) {
        user -> Uuid,
        class -> Uuid,
        display_name -> Varchar,
        role -> Int4,
    }
}

table! {
    timetables (class) {
        class -> Uuid,
        timetable -> Text,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Text,
        description -> Varchar,
    }
}

joinable!(classes -> users (owner));
joinable!(events -> classes (class));
joinable!(events -> event_types (e_type));
joinable!(members -> classes (class));
joinable!(members -> member_roles (role));
joinable!(members -> users (user));
joinable!(timetables -> classes (class));

allow_tables_to_appear_in_same_query!(
    classes,
    event_types,
    events,
    member_roles,
    members,
    timetables,
    users,
);
