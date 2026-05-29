use diesel::{
    prelude::*,
    sql_query,
    sql_types::{Integer, Text},
};

use crate::{
    db::db_client::db_conn,
    models::stats::{
        CategoryProgressRow, CategoryProgressStat, GradeCount, GradeDistributionRow, OverviewRow,
        RecentCompletion, RecentCompletionRow, TotalCompletedRow, UserStatistics,
    },
};

pub fn get_user_statistics(user_id: i32, difficulty: String) -> Result<UserStatistics, String> {
    let conn = &mut db_conn();

    let overview = sql_query(
        "
        SELECT
            COALESCE(AVG(wpm), 0.0) AS avg_wpm,
            COALESCE(AVG(accuracy), 0.0) AS avg_accuracy,
            COALESCE(MAX(wpm), 0.0) AS best_wpm,
            COALESCE(SUM(duration_seconds), 0) AS total_time_seconds,
            COUNT(*) AS total_completions
        FROM completions
        WHERE user_id = ?
        ",
    )
    .bind::<Integer, _>(user_id)
    .get_result::<OverviewRow>(conn)
    .map_err(|e| format!("Error loading statistics overview: {:?}", e))?;

    let total_completed = sql_query(
        "
        SELECT COUNT(DISTINCT lesson_id) AS total_lessons_completed
        FROM completions
        WHERE user_id = ?
        ",
    )
    .bind::<Integer, _>(user_id)
    .get_result::<TotalCompletedRow>(conn)
    .map_err(|e| format!("Error loading completed lessons count: {:?}", e))?;

    let grade_distribution_rows = sql_query(
        "
        SELECT grade, COUNT(*) AS count
        FROM completions
        WHERE user_id = ?
        GROUP BY grade
        ORDER BY grade ASC
        ",
    )
    .bind::<Integer, _>(user_id)
    .load::<GradeDistributionRow>(conn)
    .map_err(|e| format!("Error loading grade distribution: {:?}", e))?;

    let category_progress_rows = sql_query(
        "
        SELECT
            li.category AS category,
            COUNT(li.id) AS total_lessons,
            COUNT(DISTINCT c.lesson_id) AS completed_lessons
        FROM lesson_items li
        LEFT JOIN completions c
            ON c.lesson_id = li.id AND c.user_id = ?
        WHERE li.is_test = FALSE
          AND li.difficulty = ?
        GROUP BY li.category
        ORDER BY CASE li.category
            WHEN 'home-left' THEN 1
            WHEN 'home-right' THEN 2
            WHEN 'home-combined' THEN 3
            WHEN 'top-row' THEN 4
            WHEN 'bottom-row' THEN 5
            WHEN 'row-transitions' THEN 6
            WHEN 'punctuation' THEN 7
            WHEN 'common-words' THEN 8
            WHEN 'numbers-row' THEN 9
            WHEN 'speed-drills' THEN 10
            ELSE 999
        END
        ",
    )
    .bind::<Integer, _>(user_id)
    .bind::<Text, _>(difficulty)
    .load::<CategoryProgressRow>(conn)
    .map_err(|e| format!("Error loading category progress: {:?}", e))?;

    let recent_rows = sql_query(
        "
        SELECT
            c.id,
            c.lesson_id,
            li.title AS lesson_title,
            li.category,
            c.wpm,
            c.accuracy,
            c.grade,
            c.duration_seconds,
            strftime('%Y-%m-%dT%H:%M:%SZ', c.completed_at) AS completed_at
        FROM completions c
        INNER JOIN lesson_items li ON li.id = c.lesson_id
        WHERE c.user_id = ?
        ORDER BY c.completed_at DESC
        LIMIT 10
        ",
    )
    .bind::<Integer, _>(user_id)
    .load::<RecentCompletionRow>(conn)
    .map_err(|e| format!("Error loading recent activity: {:?}", e))?;

    let mut grade_distribution = vec![
        GradeCount {
            grade: "S".to_string(),
            count: 0,
        },
        GradeCount {
            grade: "A".to_string(),
            count: 0,
        },
        GradeCount {
            grade: "B".to_string(),
            count: 0,
        },
        GradeCount {
            grade: "C".to_string(),
            count: 0,
        },
        GradeCount {
            grade: "D".to_string(),
            count: 0,
        },
    ];

    for row in grade_distribution_rows {
        if let Some(slot) = grade_distribution
            .iter_mut()
            .find(|item| item.grade == row.grade)
        {
            slot.count = row.count;
        }
    }

    let category_progress = category_progress_rows
        .into_iter()
        .map(|row| CategoryProgressStat {
            category: row.category,
            completed_lessons: row.completed_lessons,
            total_lessons: row.total_lessons,
        })
        .collect::<Vec<_>>();

    let recent_activity = recent_rows
        .into_iter()
        .map(|row| RecentCompletion {
            id: row.id,
            lesson_id: row.lesson_id,
            lesson_title: row.lesson_title,
            category: row.category,
            wpm: row.wpm,
            accuracy: row.accuracy,
            grade: row.grade,
            duration_seconds: row.duration_seconds,
            completed_at: row.completed_at,
        })
        .collect::<Vec<_>>();

    Ok(UserStatistics {
        avg_wpm: overview.avg_wpm,
        avg_accuracy: overview.avg_accuracy,
        best_wpm: overview.best_wpm,
        total_time_seconds: overview.total_time_seconds,
        total_completions: overview.total_completions,
        total_lessons_completed: total_completed.total_lessons_completed,
        grade_distribution,
        category_progress,
        recent_activity,
    })
}
