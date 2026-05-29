use diesel::{prelude::*, sql_types::{BigInt, Double, Integer, Text}};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct GradeCount {
    pub grade: String,
    pub count: i64,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct CategoryProgressStat {
    pub category: String,
    pub completed_lessons: i64,
    pub total_lessons: i64,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RecentCompletion {
    pub id: i32,
    pub lesson_id: i32,
    pub lesson_title: String,
    pub category: String,
    pub wpm: f64,
    pub accuracy: f64,
    pub grade: String,
    pub duration_seconds: i32,
    pub completed_at: String,
}

#[derive(Serialize, Deserialize, Debug, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct UserStatistics {
    pub avg_wpm: f64,
    pub avg_accuracy: f64,
    pub best_wpm: f64,
    pub total_time_seconds: i64,
    pub total_completions: i64,
    pub total_lessons_completed: i64,
    pub grade_distribution: Vec<GradeCount>,
    pub category_progress: Vec<CategoryProgressStat>,
    pub recent_activity: Vec<RecentCompletion>,
}

#[derive(QueryableByName)]
pub struct OverviewRow {
    #[diesel(sql_type = Double)]
    pub avg_wpm: f64,
    #[diesel(sql_type = Double)]
    pub avg_accuracy: f64,
    #[diesel(sql_type = Double)]
    pub best_wpm: f64,
    #[diesel(sql_type = BigInt)]
    pub total_time_seconds: i64,
    #[diesel(sql_type = BigInt)]
    pub total_completions: i64,
}

#[derive(QueryableByName)]
pub struct TotalCompletedRow {
    #[diesel(sql_type = BigInt)]
    pub total_lessons_completed: i64,
}

#[derive(QueryableByName)]
pub struct GradeDistributionRow {
    #[diesel(sql_type = Text)]
    pub grade: String,
    #[diesel(sql_type = BigInt)]
    pub count: i64,
}

#[derive(QueryableByName)]
pub struct CategoryProgressRow {
    #[diesel(sql_type = Text)]
    pub category: String,
    #[diesel(sql_type = BigInt)]
    pub completed_lessons: i64,
    #[diesel(sql_type = BigInt)]
    pub total_lessons: i64,
}

#[derive(QueryableByName)]
pub struct RecentCompletionRow {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Integer)]
    pub lesson_id: i32,
    #[diesel(sql_type = Text)]
    pub lesson_title: String,
    #[diesel(sql_type = Text)]
    pub category: String,
    #[diesel(sql_type = Double)]
    pub wpm: f64,
    #[diesel(sql_type = Double)]
    pub accuracy: f64,
    #[diesel(sql_type = Text)]
    pub grade: String,
    #[diesel(sql_type = Integer)]
    pub duration_seconds: i32,
    #[diesel(sql_type = Text)]
    pub completed_at: String,
}