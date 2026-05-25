PRAGMA foreign_keys = ON;

CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    avatar VARCHAR NOT NULL DEFAULT 'fox',
    language VARCHAR NOT NULL DEFAULT 'ENGLISH' CHECK (language IN ('ENGLISH', 'FRENCH')),
    lesson_difficulty VARCHAR NOT NULL DEFAULT 'BEGINNER' CHECK (
        lesson_difficulty IN ('BEGINNER', 'INTERMEDIATE', 'ADVANCED')
    ),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_active TIMESTAMP
);

CREATE TABLE lesson_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    difficulty TEXT NOT NULL CHECK (difficulty IN ('BEGINNER', 'INTERMEDIATE', 'ADVANCED')),
    language TEXT NOT NULL CHECK (language IN ('ENGLISH', 'FRENCH')),
    category TEXT NOT NULL,
    is_test BOOLEAN NOT NULL DEFAULT FALSE,
    parent_lesson_id INTEGER NULL,
    passing_wpm INTEGER NULL,
    accuracy_threshold INTEGER NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_lesson_id) REFERENCES lesson_items (id)
);

CREATE TABLE user_completed_lessons (
    user_id INTEGER NOT NULL,
    lesson_id INTEGER NOT NULL,
    PRIMARY KEY (user_id, lesson_id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (lesson_id) REFERENCES lesson_items (id) ON DELETE CASCADE
);

INSERT INTO lesson_items (id, title, content, difficulty, language, category, is_test, parent_lesson_id, passing_wpm, accuracy_threshold)
VALUES
  (1, 'Lesson 1: A and S', 'aa ss aa ss as sa as sa aaa sss aas ssa asa sas', 'BEGINNER', 'ENGLISH', 'home-left', FALSE, NULL, NULL, NULL),
  (2, 'Lesson 2: D and F', 'dd ff dd ff df fd df fd ddd fff ddf ffd dfd fdf', 'BEGINNER', 'ENGLISH', 'home-left', FALSE, NULL, NULL, NULL),
  (3, 'Lesson 3: All Left Home', 'asdf fdsa asdf fdsa asd fds sdf dfs ads fad sad', 'BEGINNER', 'ENGLISH', 'home-left', FALSE, NULL, NULL, NULL),
  (4, 'Left Home Row Test', 'sad fad ads das fads dads adds sads fads dads sad fads', 'BEGINNER', 'ENGLISH', 'home-left', TRUE, 3, 20, 88),
  (5, 'Lesson 1: J and K', 'jj kk jj kk jk kj jk kj jjj kkk jjk kkj jkj kjk', 'BEGINNER', 'ENGLISH', 'home-right', FALSE, NULL, NULL, NULL),
  (6, 'Lesson 2: L and Semicolon', 'll ;; ll ;; l; ;l l; ;l lll ;;; ll; ;;l l;l ;l;', 'BEGINNER', 'ENGLISH', 'home-right', FALSE, NULL, NULL, NULL),
  (7, 'Lesson 3: All Right Home', 'jkl; ;lkj jkl; ;lkj jkl lkj kl; l;j ;jk jkl; ;lkj', 'BEGINNER', 'ENGLISH', 'home-right', FALSE, NULL, NULL, NULL),
  (8, 'Right Home Row Test', 'jkl; ;lkj kljl j;kl ljk; ;jlk kj;l ljk; jkl;', 'BEGINNER', 'ENGLISH', 'home-right', TRUE, 7, 20, 88),
  (9, 'Lesson 1: Both Hands', 'asdf jkl; asdf jkl; asdfjkl; ;lkjfdsa', 'BEGINNER', 'ENGLISH', 'home-combined', FALSE, NULL, NULL, NULL),
  (10, 'Lesson 2: Home Row Words', 'ask all fall fall like silk disk flask lads lass', 'BEGINNER', 'ENGLISH', 'home-combined', FALSE, NULL, NULL, NULL),
  (11, 'Lesson 3: Sentences', 'a lad asks a lass all fall flask if a disk falls', 'BEGINNER', 'ENGLISH', 'home-combined', FALSE, NULL, NULL, NULL),
  (12, 'Home Row Combined Test', 'ask all fall flask disk lads lass silk a lad asks a lass all fall', 'BEGINNER', 'ENGLISH', 'home-combined', TRUE, 11, 20, 88),
  (13, 'Lesson 1: Q W E R', 'qq ww ee rr qw we er rq qwe ewq wer rew qwer', 'BEGINNER', 'ENGLISH', 'top-row', FALSE, NULL, NULL, NULL),
  (14, 'Lesson 2: T Y U I O P', 'tt yy uu ii oo pp ty yu ui io op pt yui iuy', 'BEGINNER', 'ENGLISH', 'top-row', FALSE, NULL, NULL, NULL),
  (15, 'Lesson 3: Top Row Words', 'your type were quite power write worry pretty top row', 'BEGINNER', 'ENGLISH', 'top-row', FALSE, NULL, NULL, NULL),
  (16, 'Top Row Test', 'your type write power worry pretty quiet tower proper', 'BEGINNER', 'ENGLISH', 'top-row', TRUE, 15, 20, 88),
  (17, 'Lesson 1: Z X C V', 'zz xx cc vv zx xc cv vz zxc xcv cvz vxz zxcv', 'BEGINNER', 'ENGLISH', 'bottom-row', FALSE, NULL, NULL, NULL),
  (18, 'Lesson 2: B N M', 'bb nn mm bn nm mb bnm nmb mbn bnm nmb bbn nnm mmb', 'BEGINNER', 'ENGLISH', 'bottom-row', FALSE, NULL, NULL, NULL),
  (19, 'Lesson 3: Bottom Row Words', 'mix can vex box zinc back move comb verb next', 'BEGINNER', 'ENGLISH', 'bottom-row', FALSE, NULL, NULL, NULL),
  (20, 'Bottom Row Test', 'box mix can vex zinc move comb next back verb', 'BEGINNER', 'ENGLISH', 'bottom-row', TRUE, 19, 20, 88),
  (21, 'Lesson 1: Home to Top', 'type wake rope just fire half gate week sake tale', 'BEGINNER', 'ENGLISH', 'row-transitions', FALSE, NULL, NULL, NULL),
  (22, 'Lesson 2: Home to Bottom', 'band came have back zone calm made came back band', 'BEGINNER', 'ENGLISH', 'row-transitions', FALSE, NULL, NULL, NULL),
  (23, 'Lesson 3: All Rows', 'the quick brown fox jump over lazy dog pack hex', 'BEGINNER', 'ENGLISH', 'row-transitions', FALSE, NULL, NULL, NULL),
  (24, 'Row Transitions Test', 'the quick brown fox jumps over the lazy dog', 'BEGINNER', 'ENGLISH', 'row-transitions', TRUE, 23, 20, 88),
  (25, 'Lesson 1: Period and Comma', 'end. here, stop. wait, go. yes, no. come, stay.', 'BEGINNER', 'ENGLISH', 'punctuation', FALSE, NULL, NULL, NULL),
  (26, 'Lesson 2: Apostrophe and Quotes', 'don''t won''t can''t it''s he''s she''s they''re we''re', 'BEGINNER', 'ENGLISH', 'punctuation', FALSE, NULL, NULL, NULL),
  (27, 'Lesson 3: Full Sentences', 'Hello, world! How are you? I''m fine, thank you. Let''s go.', 'BEGINNER', 'ENGLISH', 'punctuation', FALSE, NULL, NULL, NULL),
  (28, 'Punctuation Test', 'Hello! I''m happy. Don''t stop, keep going. Are you ready? Let''s type!', 'BEGINNER', 'ENGLISH', 'punctuation', TRUE, 27, 20, 88),
  (29, 'Lesson 1: Top 25 Words', 'the be to of and a in that have it for not on with he as you do at this', 'INTERMEDIATE', 'ENGLISH', 'common-words', FALSE, NULL, NULL, NULL),
  (30, 'Lesson 2: Common Verbs', 'come say make know take go see call try ask work feel live play run turn', 'INTERMEDIATE', 'ENGLISH', 'common-words', FALSE, NULL, NULL, NULL),
  (31, 'Lesson 3: Common Sentences', 'I think you should know that we will try to make this work for everyone here.', 'INTERMEDIATE', 'ENGLISH', 'common-words', FALSE, NULL, NULL, NULL),
  (32, 'Common Words Test', 'the quick brown fox jumps over the lazy dog and every other good thing in this world', 'INTERMEDIATE', 'ENGLISH', 'common-words', TRUE, 31, 28, 90),
  (33, 'Lesson 1: 1 2 3 4 5', '1 2 3 4 5 12 23 34 45 123 234 345 1234 2345 12345', 'INTERMEDIATE', 'ENGLISH', 'numbers-row', FALSE, NULL, NULL, NULL),
  (34, 'Lesson 2: 6 7 8 9 0', '6 7 8 9 0 67 78 89 90 678 789 890 6789 7890 67890', 'INTERMEDIATE', 'ENGLISH', 'numbers-row', FALSE, NULL, NULL, NULL),
  (35, 'Lesson 3: Numbers and Words', 'I have 3 cats and 2 dogs. She ran 5 miles in 42 minutes. Call 911.', 'INTERMEDIATE', 'ENGLISH', 'numbers-row', FALSE, NULL, NULL, NULL),
  (36, 'Numbers Test', '12345 67890 100 200 300 400 500 1000 2000 3000 99 88 77 66 55', 'INTERMEDIATE', 'ENGLISH', 'numbers-row', TRUE, 35, 28, 90),
  (37, 'Speed Drill 1: Short Bursts', 'the the the and and and you you you that that that this this this with with with', 'ADVANCED', 'ENGLISH', 'speed-drills', FALSE, NULL, NULL, NULL),
  (38, 'Speed Drill 2: Common Phrases', 'as soon as possible at the same time in order to take a look make sure in addition', 'ADVANCED', 'ENGLISH', 'speed-drills', FALSE, NULL, NULL, NULL),
  (39, 'Speed Drill 3: Full Paragraph', 'Practice makes perfect. The more you type, the faster you become. Focus on accuracy first, then speed will follow naturally over time.', 'ADVANCED', 'ENGLISH', 'speed-drills', FALSE, NULL, NULL, NULL),
  (40, 'Speed Drill Final Test', 'The only way to get better at typing is to type every single day. Set a goal, track your progress, and celebrate every improvement you make.', 'ADVANCED', 'ENGLISH', 'speed-drills', TRUE, 39, 35, 92);
