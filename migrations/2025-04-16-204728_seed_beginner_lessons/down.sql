-- This file should undo anything in `up.sql`
-- Down migration: remove seeded lesson_items entries
BEGIN;

-- Delete tests first (is_test = TRUE)
DELETE FROM lesson_items
WHERE is_test = TRUE
  AND title IN (
    'Home Row Left Hand Proficiency',
    'Home Row Right Hand Proficiency',
    'Full Home Row Mastery',
    'Top Row Left Hand Test',
    'Top Row Right Hand Test',
    'Top Row Combined Test',
    'Bottom Row Left Hand Test',
    'Bottom Row Right Hand Test',
    'Full Keyboard Challenge',
    'Row Transition Proficiency',
    'Punctuation Basics Proficiency'
  );

-- Delete lessons (is_test = FALSE)
DELETE FROM lesson_items
WHERE is_test = FALSE
  AND title IN (
    'Home Row: Left Hand (ASDF)',
    'ASDF Lesson 2',
    'ASDF Lesson 3',
    'ASDF Lesson 4',
    'Home Row: Right Hand (JKL;)',
    'JKL; Lesson 2',
    'JKL; Lesson 3',
    'JKL; Lesson 4',
    'Home Row: Combined (ASDF JKL;)',
    'Home Row Combined 2',
    'Home Row Combined 3',
    'Row Transition Drill: Top ↔ Home',
    'Row Transition Drill: Home ↔ Bottom',
    'Top Row: Left Hand (QWER)',
    'QWER Lesson 2',
    'Top Row: Right Hand (UIOP)',
    'UIOP Lesson 2',
    'Top Row + Home: Combined',
    'Top Row + Home: Advanced',
    'Bottom Row: Left Hand (ZXCV)',
    'ZXCV Lesson 2',
    'Bottom Row: Right Hand (NM,.)',
    'NM,. Lesson 2',
    'Punctuation Practice 1',
    'Full Keyboard: Basic',
    'Full Keyboard: Advanced'
  );

COMMIT;
