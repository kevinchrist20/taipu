-- This file should undo anything in `up.sql`
CREATE TABLE lessons (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title VARCHAR NOT NULL,
    difficulty VARCHAR NOT NULL,
    content TEXT NOT NULL,
    language TEXT DEFAULT 'English' NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);