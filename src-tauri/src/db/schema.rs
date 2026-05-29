// @generated automatically by Diesel CLI.

diesel::table! {
    completions (id) {
        id -> Integer,
        user_id -> Integer,
        lesson_id -> Integer,
        wpm -> Double,
        accuracy -> Double,
        grade -> Text,
        duration_seconds -> Integer,
        completed_at -> Timestamp,
    }
}

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

diesel::joinable!(completions -> lesson_items (lesson_id));
diesel::joinable!(completions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(completions, lesson_items, users,);
