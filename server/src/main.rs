use anyhow::Result;
use axum::{routing::get, Router};
use sqlx::{Executor, SqlitePool};

#[tokio::main]
async fn main() -> Result<()> {
    // setup db pool
    let db_file = "db.sqlite"; // replace with env var
    let conn_str = format!("sqlite:{db_file}?mode=rwc"); // read, write, create
    let pool = SqlitePool::connect(&conn_str).await?;

    // setup db tables
    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )
        "#,
    )
    .await?;

    // setup routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(pool);

    // start server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
