// @generated automatically by Diesel CLI.

diesel::table! {
    lessons (id) {
        id -> Integer,
        title -> Text,
        content -> Text,
        difficulty -> Text,
        language -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tests (id) {
        id -> Integer,
        lesson_id -> Integer,
        title -> Text,
        content -> Text,
        passing_wpm -> Integer,
        accuracy_threshold -> Integer,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        username -> Text,
        created_at -> Nullable<Timestamp>,
        last_active -> Nullable<Timestamp>,
        keyboard_type -> Nullable<Text>,
        language -> Nullable<Text>,
        theme -> Nullable<Text>,
        lesson_difficulty -> Nullable<Text>,
    }
}

diesel::joinable!(tests -> lessons (lesson_id));

diesel::allow_tables_to_appear_in_same_query!(
    lessons,
    tests,
    users,
);
