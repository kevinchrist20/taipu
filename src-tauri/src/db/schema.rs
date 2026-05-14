// @generated automatically by Diesel CLI.

diesel::table! {
    lesson_items (id) {
        id -> Integer,
        title -> Text,
        content -> Text,
        difficulty -> Text,
        language -> Text,
        category -> Text,
        is_test -> Bool,
        parent_lesson_id -> Nullable<Integer>,
        passing_wpm -> Nullable<Integer>,
        accuracy_threshold -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        avatar -> Text,
        language -> Text,
        lesson_difficulty -> Text,
        created_at -> Nullable<Timestamp>,
        last_active -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_completed_lessons (user_id, lesson_id) {
        user_id -> Integer,
        lesson_id -> Integer,
    }
}

diesel::joinable!(user_completed_lessons -> lesson_items (lesson_id));
diesel::joinable!(user_completed_lessons -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(lesson_items, user_completed_lessons,);
