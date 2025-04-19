
mod collector;
mod web;
mod api;
mod commands;

use std::net::SocketAddr;
use axum::Extension;
use axum::routing::get;
use axum::Router;
use sqlx::SqlitePool;
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState{
    pool: SqlitePool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool  = SqlitePool::connect(&db_url).await?;
    let state = AppState { pool };

    // spawn your collector thread, passing in a clone of the pool
    let _handle = tokio::spawn(collector::data_collector(state.pool.clone()));

    // now build your router with typed state
    let app = Router::new()
        .route("/", get(web::index))
        .route("/collector.html", get(web::collector))
        .route("/api/all", get(api::show_all))
        .route("/api/collectors", get(api::show_collectors))
        .route("/api/collector/{uuid}", get(api::collector_data))
        .route("/api/collector/{uuid}/shutdown", get(api::shutdown_collector))
        .with_state(state);

    // bind a Tokio TcpListener
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    // hand it off to axum::serve
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

