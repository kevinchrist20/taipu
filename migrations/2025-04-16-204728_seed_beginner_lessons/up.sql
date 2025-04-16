-- Your SQL goes here
-- Seed script for unified lesson_items table
-- Lessons (is_test = FALSE)
INSERT INTO
    lesson_items (
        title,
        content,
        difficulty,
        language,
        category,
        is_test
    )
VALUES
    (
        'Home Row: Left Hand (ASDF)',
        'as df sa ad fs ds af sd da fa sf fd as ds af sa df ad fs sa df af sd ds as fa sf ad fs da af ds sf as fd sa df af sd ad',
        'BEGINNER',
        'ENGLISH',
        'home-left',
        FALSE
    ),
    (
        'ASDF Lesson 2',
        'asd dsa fas sdf afd fds sad das fsa ads dfs asf sfd afd dfa sad fas asd sdf fds afd das fsa dfs asf dsa afd sdf fas sad',
        'BEGINNER',
        'ENGLISH',
        'home-left',
        FALSE
    ),
    (
        'ASDF Lesson 3',
        'asdf asdf asdf asdf asdf asdf asdf asdf asdf asdf fdsa fdsa fdsa fdsa fdsa fdsa fdsa fdsa fdsa fdsa',
        'BEGINNER',
        'ENGLISH',
        'home-left',
        FALSE
    ),
    (
        'ASDF Lesson 4',
        'dfdsa dfdsa dfdsa dfdsa dfdsa dfdsa dfdsa dfdsa dfdsa fdsad fdsad fdsad fdsad fdsad fdsad fdsad fdsad fdsad',
        'BEGINNER',
        'ENGLISH',
        'home-left',
        FALSE
    ),
    (
        'Home Row: Right Hand (JKL;)',
        'jk l; lk j; kj ;l l; jk j; kl ;j lk j; l; kj ;l jk kl l; j; kl ;j lk j; ;l kj l; jk ;j kl',
        'BEGINNER',
        'ENGLISH',
        'home-right',
        FALSE
    ),
    (
        'JKL; Lesson 2',
        'jkl jkl jkl ;j; ;l; ;k; jkl; jkl; jkl; ;lkj ;lkj ;lkj jl jl jl k; k; k; lk lk lk j; j; j;',
        'BEGINNER',
        'ENGLISH',
        'home-right',
        FALSE
    ),
    (
        'JKL; Lesson 3',
        'jkl; jkl; jkl; ;lkj ;lkj ;lkj kl;j kl;j kl;j j;lk j;lk j;lk ljk; ljk; ljk; ;jkl ;jkl ;jkl',
        'BEGINNER',
        'ENGLISH',
        'home-right',
        FALSE
    ),
    (
        'JKL; Lesson 4',
        'jkl jkl jkl; jkl jkl; jkl; ;lkj ;lkj ;lkj jkl; jkl jkl jkl jkl; jkl jkl; jkl; jkl; jkl; jkl; jkl; jkl; jkl; jkl; ;lkj ;lkj ;lkj ;lkj ;lkj jkl ;lk jkl ;lk lkj jkl jkl ;lk lkj jkl jkl lkj jkl ;lk lkj jkl jkl',
        'BEGINNER',
        'ENGLISH',
        'home-right',
        FALSE
    ),
    (
        'Home Row: Combined (ASDF JKL;)',
        'as jk df l; sal jdk fad l;k jak sal dak f;l sja dkf l;a fs jk ad l; fs jak dls f;a',
        'BEGINNER',
        'ENGLISH',
        'home-combined',
        FALSE
    ),
    (
        'Home Row Combined 2',
        'ask jad flak said jak; flash fall; dads; jassklad; adj; fads; lass; jak; dash; flask; salad',
        'BEGINNER',
        'ENGLISH',
        'home-combined',
        FALSE
    ),
    (
        'Home Row Combined 3',
        'a slack fjask; ajakad lad; a salad; all fall; a lass; ask dad; a fad; a flask; a fall; a klassj',
        'BEGINNER',
        'ENGLISH',
        'home-combined',
        FALSE
    ),
    (
        'Row Transition Drill: Top ↔ Home',
        'q a w s e d r f t g y h u j i k o l p ;',
        'BEGINNER',
        'ENGLISH',
        'transition-top-home',
        FALSE
    ),
    (
        'Row Transition Drill: Home ↔ Bottom',
        'a z s x d c f v g b h n j m k , l . ; /',
        'BEGINNER',
        'ENGLISH',
        'transition-home-bottom',
        FALSE
    ),
    (
        'Top Row: Left Hand (QWER)',
        'qw er qe wr re wq er qw re qw er re wq er qw re wq qe rw er qw re wq er qw re qw er',
        'BEGINNER',
        'ENGLISH',
        'top-left',
        FALSE
    ),
    (
        'QWER Lesson 2',
        'qwer qwer qwer rewq rewq rewq qrew qrew qrew werq werq werq qwer rewq qrew werq rewq qwer',
        'BEGINNER',
        'ENGLISH',
        'top-left',
        FALSE
    ),
    (
        'Top Row: Right Hand (UIOP)',
        'ui op ui po iu op ui op oi up pi uo iu op po ui op iu po ui oi up iu op ui po op ui',
        'BEGINNER',
        'ENGLISH',
        'top-right',
        FALSE
    ),
    (
        'UIOP Lesson 2',
        'uiop uiop uiop poiu poiu poiu upio oupi upoi poiu uiop poui ipuo upoi poiu uiop oipu',
        'BEGINNER',
        'ENGLISH',
        'top-right',
        FALSE
    ),
    (
        'Top Row + Home: Combined',
        'quip pals dark joke flap weir usual drop fork spill wake quad oil perk far jape disk ask',
        'BEGINNER',
        'ENGLISH',
        'top-home-combined',
        FALSE
    ),
    (
        'Top Row + Home: Advanced',
        'gold flakes wire apis disks leaf wake push oil repair quip sale dark jaws fluid operator',
        'BEGINNER',
        'ENGLISH',
        'top-home-combined',
        FALSE
    ),
    (
        'Bottom Row: Left Hand (ZXCV)',
        'zx cv zc xv zx cv zc vx xc vz zc xv zx cv zx vc xz cv zx cv zc xv zv cx zx cv zc xv',
        'BEGINNER',
        'ENGLISH',
        'bottom-left',
        FALSE
    ),
    (
        'ZXCV Lesson 2',
        'zxcv zxcv zxcv vcxz vcxz vcxz zvxc cxvz xczv vzxc czvx xvzc zxcv vcxz zvxc cxvz xvzc',
        'BEGINNER',
        'ENGLISH',
        'bottom-left',
        FALSE
    ),
    (
        'Bottom Row: Right Hand (NM,.)',
        'nm ,. nm ., mn ,. nm ,. n, m. ,m .n mn ,. ., nm ,. mn ., nm n, m. mn ,. nm ., ,. nm',
        'BEGINNER',
        'ENGLISH',
        'bottom-right',
        FALSE
    ),
    (
        'NM,. Lesson 2',
        'nm,. nm,. nm,. .,mn .,mn .,mn n.,m ,nm. m,n. .,mn nm,. .m,n n,.m m,n. .,mn nm,. m,n.',
        'BEGINNER',
        'ENGLISH',
        'bottom-right',
        FALSE
    ),
    (
        'Punctuation Practice 1',
        'a. a! a? a, a. b! c? d. e! f? g. h! i? j. k! l? ; a. b. c, d! e? f; g, h. i! j;',
        'BEGINNER',
        'ENGLISH',
        'punctuation',
        FALSE
    ),
    (
        'Full Keyboard: Basic',
        'quick vexing wolf jumps amazed by plot. the five boxing wizards jump quickly. lazy dogs are jumping over the fence.',
        'BEGINNER',
        'ENGLISH',
        'full-keyboard',
        FALSE
    ),
    (
        'Full Keyboard: Advanced',
        'pack my box with five dozen liquor jugs. how quickly daft jumping zebras vex. quiet jays form blazing wrecks. Wind blows, leaves fall, yet the fox stays swift.',
        'BEGINNER',
        'ENGLISH',
        'full-keyboard',
        FALSE
    );

-- Tests (is_test = TRUE)
-- Home Row Left Hand Test
INSERT INTO
    lesson_items (
        title,
        content,
        difficulty,
        language,
        category,
        is_test,
        parent_lesson_id,
        passing_wpm,
        accuracy_threshold
    )
SELECT
    'Home Row Left Hand Proficiency',
    'sad dad fast fads ads a sad dad adds a fad a sad dad adds a fad ads and fads add sass a fast sad dad adds sad fads fast ads fade as dad adds a fad',
    difficulty,
    language,
    category,
    TRUE,
    id,
    15,
    85
FROM
    lesson_items
WHERE
    title = 'ASDF Lesson 4';

-- Home Row Right Hand Test
INSERT INTO
    lesson_items (
        title,
        content,
        difficulty,
        language,
        category,
        is_test,
        parent_lesson_id,
        passing_wpm,
        accuracy_threshold
    )
SELECT
    'Home Row Right Hand Proficiency',
    'jkl; jklj jkjl kljk jkl; lkj; jl kl j; l; kj lk jl jk ;j l; kj jkl; ;lkj jlk; jl;k j;lk ;lkj jjkl jkl;;',
    difficulty,
    language,
    category,
    TRUE,
    id,
    15,
    85
FROM
    lesson_items
WHERE
    title = 'JKL; Lesson 4';

-- Full Home Row Mastery
INSERT INTO
    lesson_items (
        title,
        content,
        difficulty,
        language,
        category,
        is_test,
        parent_lesson_id,
        passing_wpm,
        accuracy_threshold
    )
SELECT
    'Full Home Row Mastery',
    'jak falls alas; a salad; ask dad; all shall jak; as a lad; a flask; a fad; a lass shall fall; a jak; a dad; a flask; askjads;',
    'BEGINNER',
    'ENGLISH',
    'home-combined',
    TRUE,
    id,
    20,
    90
FROM
    lesson_items
WHERE
    title = 'Home Row Combined 3';

-- Top Row Left Hand Test
INSERT INTO
    lesson_items (
        title,
        content,
        difficulty,
        language,
        category,
        is_test,
        parent_lesson_id,
        passing_wpm,
        accuracy_threshold
    )
SELECT
    'Top Row Left Hand Test',
    'qwer were we we are as red as were we qwer we are aware we were well fed as we were free qwer a sawed reed was rewarded',
    difficulty,
    language,
    category,
    TRUE,
    id,
    20,
    85
FROM
    lesson_items
WHERE
    title = 'QWER Lesson 2';

-- Top Row Right Hand Test
INSERT INTO
    lesson_items (
        title,
        content,
        difficulty,
        language,
        category,
        is_test,
        parent_lesson_id,
        passing_wpm,
        accuracy_threshold
    )
SELECT
    'Top Row Right Hand Test',
    'up poi io pup pop our oil poor pop up ui oi op io uo pi pop up or oil our poiupoiu ioup oupi',
    difficulty,
    language,
    category,
    TRUE,
    id,
    20,
    85
FROM
    lesson_items
WHERE
    title = 'UIOP Lesson 2';

-- Top Row Combined Test
INSERT INTO
    lesson_items (
        title,
        content,
        difficulty,
        language,
        category,
        is_test,
        parent_lesson_id,
        passing_wpm,
        accuracy_threshold
    )
SELECT
    'Top Row Combined Test',
    'we proudly work; a quiet person; look upward; paid repair for older jeep; dark liquor pour; quip for joke; also repair work; op',
    difficulty,
    language,
    category,
    TRUE,
    id,
    25,
    85
FROM
    lesson_items
WHERE
    title = 'Top Row + Home: Advanced';

-- Bottom Row Left Hand Test
INSERT INTO
    lesson_items (
        title,
        content,
        difficulty,
        language,
        category,
        is_test,
        parent_lesson_id,
        passing_wpm,
        accuracy_threshold
    )
SELECT
    'Bottom Row Left Hand Test',
    'zap zest zeal vex cave axe fez viz fizzed crave as vase excel save a crave zap vex zest vase excel czar facade far',
    difficulty,
    language,
    category,
    TRUE,
    id,
    25,
    85
FROM
    lesson_items
WHERE
    title = 'ZXCV Lesson 2';

-- Bottom Row Right Hand Test
INSERT INTO
    lesson_items (
        title,
        content,
        difficulty,
        language,
        category,
        is_test,
        parent_lesson_id,
        passing_wpm,
        accuracy_threshold
    )
SELECT
    'Bottom Row Right Hand Test',
    'man, no, an, am, nan, mom, men, jam, ham, nam, nom, nem, man, nam, mom, no, man, jam, an, man, no, man, am, nom, jam, ham',
    difficulty,
    language,
    category,
    TRUE,
    id,
    25,
    85
FROM
    lesson_items
WHERE
    title = 'NM,. Lesson 2';

-- Full Keyboard Challenge Test
INSERT INTO
    lesson_items (
        title,
        content,
        difficulty,
        language,
        category,
        is_test,
        parent_lesson_id,
        passing_wpm,
        accuracy_threshold
    )
SELECT
    'Full Keyboard Challenge',
    'the five boxing wizards jump quickly; pack my box with five dozen liquor jugs; how vexingly quick daft zebras jump',
    difficulty,
    language,
    category,
    TRUE,
    id,
    30,
    90
FROM
    lesson_items
WHERE
    title = 'Full Keyboard: Advanced';

-- Row Transition Proficiency Test
INSERT INTO
    lesson_items (
        title,
        content,
        difficulty,
        language,
        category,
        is_test,
        parent_lesson_id,
        passing_wpm,
        accuracy_threshold
    )
SELECT
    'Row Transition Proficiency',
    'q a w s e d r f t g y h u j i k o l p ; a z s x d c f v g b h n j m k , l . ; /',
    difficulty,
    language,
    category,
    TRUE,
    id,
    24,
    88
FROM
    lesson_items
WHERE
    title = 'Row Transition Drill: Home ↔ Bottom';

-- Punctuation Basics Proficiency Test
INSERT INTO
    lesson_items (
        title,
        content,
        difficulty,
        language,
        category,
        is_test,
        parent_lesson_id,
        passing_wpm,
        accuracy_threshold
    )
SELECT
    'Punctuation Basics Proficiency',
    'a. b! c? d. e! f? g. h! i? j. k! l? ; a. b. c, d! e? f; g, h. i! j;',
    difficulty,
    language,
    category,
    TRUE,
    id,
    18,
    85
FROM
    lesson_items
WHERE
    title = 'Punctuation Practice 1';