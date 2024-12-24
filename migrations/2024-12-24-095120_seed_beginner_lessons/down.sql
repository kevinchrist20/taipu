-- This file should undo anything in `up.sql`
-- Delete the test associated with beginner lessons
DELETE FROM tests
WHERE title = 'Home Row Left Hand Proficiency';

-- Delete beginner lessons
DELETE FROM lessons
WHERE title IN (
    'Home Row: Left Hand (ASDF)',
    'ASDF Lesson 2',
    'ASDF Lesson 3',
    'ASDF Lesson 4'
);