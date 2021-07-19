table! {
    class (id) {
        id -> Uuid,
        owner -> Nullable<Uuid>,
        name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        timetable -> Nullable<Json>,
    }
}

table! {
    member (users, class) {
        users -> Uuid,
        class -> Uuid,
        display_name -> Nullable<Varchar>,
        role -> Nullable<Int4>,
    }
}

table! {
    member_role (id) {
        id -> Int4,
        display -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Nullable<Varchar>,
        password -> Nullable<Text>,
        description -> Nullable<Varchar>,
    }
}

joinable!(class -> users (owner));
joinable!(member -> class (class));
joinable!(member -> member_role (role));
joinable!(member -> users (users));

allow_tables_to_appear_in_same_query!(
    class,
    member,
    member_role,
    users,
);
