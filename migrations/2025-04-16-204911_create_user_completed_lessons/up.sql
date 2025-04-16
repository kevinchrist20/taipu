-- Your SQL goes here
CREATE TABLE
    IF NOT EXISTS user_completed_lessons (
        user_id INTEGER NOT NULL,
        lesson_id INTEGER NOT NULL,
        PRIMARY KEY (user_id, lesson_id),
        FOREIGN KEY (user_id) REFERENCES users (id),
        FOREIGN KEY (lesson_id) REFERENCES lesson_items (id)
    );