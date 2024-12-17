-- Your SQL goes here
CREATE TABLE
    users (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        name VARCHAR NOT NULL,
        username VARCHAR UNIQUE NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        last_active TIMESTAMP,
        keyboard_type VARCHAR DEFAULT 'QWERTY',
        language VARCHAR DEFAULT 'English',
        theme VARCHAR DEFAULT 'dark',
        lesson_difficulty VARCHAR DEFAULT 'beginner'
    );