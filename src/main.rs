use std::sync::Arc;

use axum::{routing::get, Router, extract::State};
use clap::Parser;
use map_thing_back::config::Config;
use sqlx::postgres::PgPoolOptions;

pub struct AppState {
    db: sqlx::PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let config = Config::parse();

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.db_url)
        .await
        .unwrap();

    sqlx::migrate!().run(&db).await?;

    let app_state = Arc::new(AppState { db });

    let app = Router::new()
        .route("/", get(index))
        .route("/user", get(get_user))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn index() -> &'static str {
    "Hello, World!"
}

async fn get_user(State(app_state): State<Arc<AppState>>) -> String {
    sqlx::query!("SELECT name FROM users WHERE id = $1", 1)
        .fetch_one(&app_state.db)
        .await
        .unwrap()
        .name
}
