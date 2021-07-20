table! {
    class (id) {
        id -> Uuid,
        owner -> Uuid,
        name -> Varchar,
        description -> Varchar,
        timetable -> Text,
    }
}

table! {
    member (user, class) {
        user -> Uuid,
        class -> Uuid,
        display_name -> Varchar,
        role -> Int4,
    }
}

table! {
    member_role (id) {
        id -> Int4,
        display -> Varchar,
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

joinable!(class -> users (owner));
joinable!(member -> class (class));
joinable!(member -> member_role (role));
joinable!(member -> users (user));

allow_tables_to_appear_in_same_query!(
    class,
    member,
    member_role,
    users,
);
