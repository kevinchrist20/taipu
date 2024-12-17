// @generated automatically by Diesel CLI.

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
