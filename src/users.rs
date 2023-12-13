use std::sync::Arc;

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

use crate::AppState;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_users).post(add_user))
        .route("/:id", get(get_users_id))
}

pub async fn get_users(State(app_state): State<Arc<AppState>>) -> Json<Vec<User>> {
    sqlx::query_as!(User, r#"SELECT * FROM users"#,)
        .fetch_all(&app_state.db)
        .await
        .unwrap()
        .into()
}

pub async fn get_users_id(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<User>, ()> {
    let user = sqlx::query_as!(User, r#"SELECT * FROM users WHERE id = $1"#, id)
        .fetch_one(&app_state.db)
        .await;

    match user {
        Ok(user) => Ok(user.into()),
        Err(_) => Err(()),
    }
}

pub async fn add_user(
    State(app_state): State<Arc<AppState>>,
    Json(user): Json<User>,
) -> Json<User> {
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (name)
        VALUES ($1)
        RETURNING *
        "#,
        user.name
    )
    .fetch_one(&app_state.db)
    .await
    .unwrap();

    user.into()
}
