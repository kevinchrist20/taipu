-- Your SQL goes here
CREATE TABLE
    users (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        name VARCHAR NOT NULL,
        username VARCHAR UNIQUE NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        last_active TIMESTAMP,
        keyboard_type VARCHAR NOT NULL DEFAULT 'QWERTY' CHECK (keyboard_type IN ('QWERTY', 'AZERTY')),
        language VARCHAR NOT NULL DEFAULT 'ENGLISH' CHECK (language IN ('ENGLISH', 'FRENCH')),
        theme VARCHAR NOT NULL DEFAULT 'LIGHT' CHECK (theme IN ('LIGHT', 'DARK')),
        lesson_difficulty VARCHAR NOT NULL DEFAULT 'BEGINNER' CHECK (
            lesson_difficulty IN ('BEGINNER', 'INTERMEDIATE', 'ADVANCED')
        ),
    );