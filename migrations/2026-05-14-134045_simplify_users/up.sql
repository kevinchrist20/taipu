-- Recreate users table with simplified schema (SQLite table-rebuild pattern)
-- FK checks are disabled at the connection level in Rust before this runs

CREATE TABLE users_new (
    id                INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name              VARCHAR NOT NULL,
    avatar            VARCHAR NOT NULL DEFAULT 'fox',
    language          VARCHAR NOT NULL DEFAULT 'ENGLISH' CHECK (language IN ('ENGLISH', 'FRENCH')),
    lesson_difficulty VARCHAR NOT NULL DEFAULT 'BEGINNER' CHECK (
        lesson_difficulty IN ('BEGINNER', 'INTERMEDIATE', 'ADVANCED')
    ),
    created_at        TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_active       TIMESTAMP
);

INSERT INTO users_new (id, name, avatar, language, lesson_difficulty, created_at, last_active)
SELECT
    id,
    name,
    'fox',
    COALESCE(language, 'ENGLISH'),
    COALESCE(lesson_difficulty, 'BEGINNER'),
    created_at,
    last_active
FROM users;

DROP TABLE users;
ALTER TABLE users_new RENAME TO users;
