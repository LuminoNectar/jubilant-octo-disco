use anyhow::Result;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use sqlx::{Executor, SqlitePool};

mod routers;

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
            id       CHAR(16) PRIMARY KEY,
            password VARCHAR(30) NOT NULL
        )
        "#,
    )
    .await?;

    // setup routes
    let app = Router::new()
        .route("/status", get(|| async { "it works! :)" }))
        .route(
            "/login",
            get(|| async {
                Html(include_str!("../public/loginPages/login.html"))
            }),
        )
        .route(
            "/existingUser",
            get(|| async {
                Html(include_str!("../public/loginPages/existingUser.html"))
            }),
        )
        .route(
            "/newUser",
            get(|| async {
                Html(include_str!("../public/loginPages/newUser.html"))
            }),
        )
        .route(
            "/metrics",
            get(|| async {
                Html(include_str!("../public/mainPages/metrics.html"))
            }),
        )
        .route(
            "/history",
            get(|| async {
                Html(include_str!("../public/mainPages/history.html"))
            }),
        )
        .route(
            "/devices",
            get(|| async {
                Html(include_str!("../public/mainPages/devices.html"))
            }),
        )
        .route(
            "/account",
            get(|| async {
                Html(include_str!("../public/mainPages/account.html"))
            }),
        )
        .merge(routers::auth::router(&pool));

    // start server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

pub struct AppError(anyhow::Error);

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}
