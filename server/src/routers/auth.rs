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
    State(_pool): State<SqlitePool>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    // TODO: implement login
    (StatusCode::CREATED, Json(user))
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
