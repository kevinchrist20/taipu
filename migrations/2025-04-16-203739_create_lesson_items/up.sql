-- Your SQL goes here
CREATE TABLE
    lesson_items (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        content TEXT NOT NULL,
        difficulty TEXT NOT NULL,
        language TEXT NOT NULL,
        category TEXT NOT NULL, -- e.g., 'home-left', 'top-right', 'punctuation'
        is_test BOOLEAN DEFAULT FALSE, -- false = lesson, true = test
        parent_lesson_id INTEGER NULL, -- for tests, link back to lesson; NULL for lessons
        passing_wpm INTEGER NULL, -- only for tests
        accuracy_threshold INTEGER NULL, -- only for tests
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );