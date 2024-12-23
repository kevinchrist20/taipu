-- Your SQL goes here
CREATE TABLE
    lessons (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        title VARCHAR NOT NULL,
        content TEXT NOT NULL,
        difficulty TEXT NOT NULL CHECK (
            difficulty IN ('BEGINNER', 'INTERMEDIATE', 'ADVANCED')
        ),
        language TEXT NOT NULL CHECK (language IN ('ENGLISH', 'FRENCH')),
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );