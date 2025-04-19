use axum::{
    extract::{State, Path},
    response::IntoResponse,
    Json,
    http::StatusCode,
};
use serde::Serialize;
use sqlx::FromRow;
use crate::AppState;
use crate::commands::add_command;
use shared_code::TaskType;


#[derive(FromRow, Debug, Serialize)]
pub struct DataPoint {
    id: i32,
    collector_id: String,
    received: i64,
    total_memory: i64,
    used_memory: i64,
    average_cpu: f32,
}

pub async fn show_all(
    State(AppState { pool }): State<AppState>
) -> Json<Vec<DataPoint>> {
    let rows = sqlx::query_as::<_, DataPoint>("SELECT * FROM timeseries")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(rows)
}

#[derive(FromRow, Debug, Serialize)]
pub struct Collector {
    id: i32,
    collector_id: String,
    last_seen: i64,
}

pub async fn show_collectors(
    State(AppState { pool }): State<AppState>
) -> Json<Vec<Collector>> {
    const SQL: &str = r#"
        SELECT DISTINCT(id) AS id,
               collector_id,
               (SELECT MAX(received)
                FROM timeseries
                WHERE collector_id = ts.collector_id)
            AS last_seen
        FROM timeseries ts
    "#;
    let rows = sqlx::query_as::<_, Collector>(SQL)
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(rows)
}

pub async fn collector_data(
    State(AppState { pool }): State<AppState>,
    Path(uuid): Path<String>,
) -> Json<Vec<DataPoint>> {
    let rows = sqlx::query_as::<_, DataPoint>(
        "SELECT * FROM timeseries WHERE collector_id = ? ORDER BY received"
    )
        .bind(&uuid)
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(rows)
}

pub async fn shutdown_collector(
    State(AppState { pool }): State<AppState>,
    Path(uuid): Path<String>,
) -> impl IntoResponse {
    // convert string → u128, enqueue the shutdown command, etc.
    let id = uuid.parse::<u128>().unwrap();
    add_command(id, TaskType::Shutdown);
    // return a 200 OK
    StatusCode::OK
}