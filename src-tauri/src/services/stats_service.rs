use diesel::{
    prelude::*,
    sql_query,
    sql_types::{Integer, Text},
};

use crate::{
    db::db_client::db_conn,
    models::stats::{
        CategoryProgressRow, CategoryProgressStat, DailyActivityRow, DailyActivityStat,
        DistinctDayRow, GradeCount, GradeDistributionRow, OverviewRow, RecentCompletion,
        RecentCompletionRow, StreakInfo, TotalCompletedRow, TreeStageInfo, UserStatistics,
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
                FROM completions c
                INNER JOIN lesson_items li ON li.id = c.lesson_id
                WHERE c.user_id = ?
                    AND li.difficulty = ?
        ",
    )
    .bind::<Integer, _>(user_id)
    .bind::<Text, _>(difficulty.as_str())
    .get_result::<OverviewRow>(conn)
    .map_err(|e| format!("Error loading statistics overview: {:?}", e))?;

    let total_completed = sql_query(
        "
        SELECT COUNT(DISTINCT lesson_id) AS total_lessons_completed
                FROM completions c
                INNER JOIN lesson_items li ON li.id = c.lesson_id
                WHERE c.user_id = ?
                    AND li.difficulty = ?
        ",
    )
    .bind::<Integer, _>(user_id)
    .bind::<Text, _>(difficulty.as_str())
    .get_result::<TotalCompletedRow>(conn)
    .map_err(|e| format!("Error loading completed lessons count: {:?}", e))?;

    let grade_distribution_rows = sql_query(
        "
        SELECT grade, COUNT(*) AS count
                FROM completions c
                INNER JOIN lesson_items li ON li.id = c.lesson_id
                WHERE c.user_id = ?
                    AND li.difficulty = ?
        GROUP BY grade
        ORDER BY grade ASC
        ",
    )
    .bind::<Integer, _>(user_id)
    .bind::<Text, _>(difficulty.as_str())
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
    .bind::<Text, _>(difficulty.as_str())
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
                    AND li.difficulty = ?
        ORDER BY c.completed_at DESC
        LIMIT 10
        ",
    )
    .bind::<Integer, _>(user_id)
    .bind::<Text, _>(difficulty.as_str())
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

    let daily_activity_rows = sql_query(
        "
        SELECT
            strftime('%Y-%m-%d', c.completed_at) AS date,
            COUNT(*) AS completions_count,
            COALESCE(AVG(c.wpm), 0.0) AS avg_wpm,
            COALESCE(AVG(c.accuracy), 0.0) AS avg_accuracy
        FROM completions c
        INNER JOIN lesson_items li ON li.id = c.lesson_id
        WHERE c.user_id = ?
            AND li.difficulty = ?
        GROUP BY strftime('%Y-%m-%d', c.completed_at)
        ORDER BY date ASC
        ",
    )
    .bind::<Integer, _>(user_id)
    .bind::<Text, _>(difficulty.as_str())
    .load::<DailyActivityRow>(conn)
    .map_err(|e| format!("Error loading daily activity: {:?}", e))?;

    let distinct_day_rows = sql_query(
        "
        SELECT DISTINCT
            strftime('%Y-%m-%d', completed_at) AS day
        FROM completions
        WHERE user_id = ?
        ORDER BY day ASC
        ",
    )
    .bind::<Integer, _>(user_id)
    .load::<DistinctDayRow>(conn)
    .map_err(|e| format!("Error loading distinct active days: {:?}", e))?;

    let distinct_days: Vec<String> = distinct_day_rows.into_iter().map(|r| r.day).collect();
    let streak_info = calculate_streaks(&distinct_days);

    let daily_trends = daily_activity_rows
        .into_iter()
        .map(|row| DailyActivityStat {
            date: row.date,
            completions_count: row.completions_count,
            avg_wpm: row.avg_wpm,
            avg_accuracy: row.avg_accuracy,
        })
        .collect::<Vec<_>>();

    let tree_stage = calculate_tree_stage(
        total_completed.total_lessons_completed,
        overview.total_completions,
        overview.avg_wpm,
        overview.avg_accuracy,
        streak_info.current_streak,
    );

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
        daily_trends,
        streak_info,
        tree_stage,
    })
}

fn calculate_streaks(dates: &[String]) -> StreakInfo {
    if dates.is_empty() {
        return StreakInfo {
            current_streak: 0,
            best_streak: 0,
            last_active_date: None,
        };
    }

    let mut parsed_dates: Vec<chrono::NaiveDate> = dates
        .iter()
        .filter_map(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
        .collect();

    parsed_dates.sort();
    parsed_dates.dedup();

    if parsed_dates.is_empty() {
        return StreakInfo {
            current_streak: 0,
            best_streak: 0,
            last_active_date: None,
        };
    }

    let last_active = *parsed_dates.last().unwrap();
    let today = chrono::Local::now().date_naive();

    let mut best_streak = 0;
    let mut current_run = 0;
    let mut prev_date: Option<chrono::NaiveDate> = None;

    for date in &parsed_dates {
        match prev_date {
            Some(prev) => {
                let diff = (*date - prev).num_days();
                if diff == 1 {
                    current_run += 1;
                } else if diff > 1 {
                    current_run = 1;
                }
            }
            None => {
                current_run = 1;
            }
        }
        if current_run > best_streak {
            best_streak = current_run;
        }
        prev_date = Some(*date);
    }

    let days_since_last = (today - last_active).num_days();
    let current_streak = if days_since_last == 0 || days_since_last == 1 {
        let mut streak = 1;
        let mut current_check = last_active;
        for date in parsed_dates.iter().rev().skip(1) {
            if (current_check - *date).num_days() == 1 {
                streak += 1;
                current_check = *date;
            } else {
                break;
            }
        }
        streak
    } else {
        0
    };

    StreakInfo {
        current_streak,
        best_streak,
        last_active_date: Some(last_active.format("%Y-%m-%d").to_string()),
    }
}

fn calculate_tree_stage(
    total_lessons_completed: i64,
    _total_completions: i64,
    avg_wpm: f64,
    avg_accuracy: f64,
    current_streak: i64,
) -> TreeStageInfo {
    let (stage, stage_name, stage_description, progress_percentage, next_milestone_hint) =
        if total_lessons_completed < 6 && avg_wpm < 30.0 {
            let progress = ((total_lessons_completed as f64) / 6.0 * 100.0).clamp(0.0, 99.0);
            (
                1,
                "Sprout".to_string(),
                "A tender green shoot taking root in the soil. Consistent practice nurtures healthy growth.".to_string(),
                progress,
                format!(
                    "Complete {} more lesson{} to sprout into a Sapling",
                    6 - total_lessons_completed,
                    if 6 - total_lessons_completed == 1 { "" } else { "s" }
                ),
            )
        } else if total_lessons_completed < 16 && avg_wpm < 45.0 {
            let progress = (((total_lessons_completed - 6) as f64) / 10.0 * 100.0).clamp(0.0, 99.0);
            (
                2,
                "Sapling".to_string(),
                "A resilient young sapling developing sturdy stems and lateral branches.".to_string(),
                progress,
                format!(
                    "Complete {} more lesson{} or achieve 45+ WPM to branch out",
                    16 - total_lessons_completed,
                    if 16 - total_lessons_completed == 1 { "" } else { "s" }
                ),
            )
        } else if total_lessons_completed < 36 && avg_wpm < 60.0 {
            let progress = (((total_lessons_completed - 16) as f64) / 20.0 * 100.0).clamp(0.0, 99.0);
            (
                3,
                "Branching Bonsai".to_string(),
                "A defined bonsai with balanced branches. Your keystroke cadence and rhythm are solidifying.".to_string(),
                progress,
                "Reach 50+ WPM with 90%+ accuracy or complete 36 lessons to grow a lush canopy".to_string(),
            )
        } else if total_lessons_completed < 60 && (avg_wpm < 75.0 || current_streak < 3) {
            let progress = (((total_lessons_completed - 36) as f64) / 24.0 * 100.0).clamp(0.0, 99.0);
            (
                4,
                "Verdant Canopy".to_string(),
                "A flourishing canopy with dense, vibrant leaves. Speed and precision are in full harmony.".to_string(),
                progress,
                "Maintain a 3+ day streak and achieve 75+ WPM to unlock Full Bloom".to_string(),
            )
        } else {
            (
                5,
                "Full Bloom".to_string(),
                "A master bonsai crowned with delicate blossoms. High discipline, muscle memory, and fluid typing achieved.".to_string(),
                100.0,
                "Mastery reached! Keep up your daily streak to nurture fresh blooms.".to_string(),
            )
        };

    let foliage_density = if avg_accuracy <= 60.0 {
        0.2
    } else {
        ((avg_accuracy - 60.0) / 40.0).clamp(0.2, 1.0)
    };

    let bloom_count = match current_streak {
        0 => 0,
        1 => 2,
        2 => 4,
        3 => 6,
        4..=6 => 8,
        _ => 12,
    };

    let total_branches = (stage * 2 + 1).min(11);

    TreeStageInfo {
        stage,
        stage_name,
        stage_description,
        progress_percentage,
        next_milestone_hint,
        foliage_density,
        bloom_count,
        total_branches,
    }
}
