use crate::AppError;
use axum::{
    extract::State, http::StatusCode, response::IntoResponse, routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

pub fn router(pool: &SqlitePool) -> Router {
    Router::new().nest(
        "/auth",
        Router::new()
            .route("/login", post(login))
            .route("/register", post(register))
            .with_state(pool.clone()),
    )
}

pub async fn login(
    State(pool): State<SqlitePool>,
    Json(user): Json<User>,
) -> Result<impl IntoResponse, AppError> {
    #[derive(sqlx::FromRow)]
    struct Row {
        count: i64,
    }
    #[derive(Serialize)]
    struct ErrorMessage {
        msg: &'static str,
    }
    let data: Row = sqlx::query_as(
        r#"
        SELECT COUNT(*) count FROM users 
        WHERE id = ? AND password = ?
        "#,
    )
    .bind(&user.id)
    .bind(&user.password)
    .fetch_one(&pool)
    .await?;
    if data.count > 0 {
        Ok((StatusCode::OK, Json(user).into_response()))
    } else {
        Ok((
            StatusCode::UNAUTHORIZED,
            Json(ErrorMessage { msg: "Bad Login!" }).into_response(),
        ))
    }
}

pub async fn register(
    State(pool): State<SqlitePool>,
) -> Result<impl IntoResponse, AppError> {
    let user = User {
        id: Uuid::new_v4().to_string(),
        password: Uuid::new_v4().to_string(),
    };
    let data: User = sqlx::query_as(
        r#"
        INSERT INTO users (id, password)
        values (?, ?)
        RETURNING id, password
        "#,
    )
    .bind(user.id)
    .bind(user.password)
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(data)))
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    id: String,
    password: String,
}
