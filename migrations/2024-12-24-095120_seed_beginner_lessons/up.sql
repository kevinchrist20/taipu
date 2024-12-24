-- Your SQL goes here
-- Insert beginner lessons
INSERT INTO lessons (title, difficulty, content, language)
VALUES
    ('Home Row: Left Hand (ASDF)', 'BEGINNER', 'as df sa ad fs ds af sd da fa sf fd as ds af sa df ad fs sa df af sd ds as fa sf ad fs da af ds sf as fd sa df af sd ad', 'ENGLISH'),
    ('ASDF Lesson 2', 'BEGINNER', 'asd dsa fas sdf afd fds sad das fsa ads dfs asf sfd afd dfa sad fas asd sdf fds afd das fsa dfs asf dsa afd sdf fas sad', 'ENGLISH'),
    ('ASDF Lesson 3', 'BEGINNER', 'asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf fdsa fdsa fdsa fdsa fdsa fdsa fdsa fdsa fdsa fdsa', 'ENGLISH'),
    ('ASDF Lesson 4', 'BEGINNER', 'dfdsa dfdsa dfdsa dfdsa dfdsa dfdsa dfdsa dfdsa dfdsa fdsad fdsad fdsad fdsad fdsad fdsad fdsad fdsad fdsad', 'ENGLISH');

-- Insert associated test for the last lesson
INSERT INTO tests (lesson_id, title, content, passing_wpm, accuracy_threshold)
SELECT id, 'Home Row Left Hand Proficiency',
       'sad dad fast fads ads a sad dad adds a fad a sad dad adds a fad ads and fads add sass a fast sad dad adds sad fads fast ads fade as dad adds a fad', 15, 85
FROM lessons
WHERE title = 'ASDF Lesson 4';