use std::sync::Arc;

use axum::{routing::get, Router};
use clap::Parser;
use map_thing_back::{config::Config, routes, users, AppState};
use sqlx::postgres::PgPoolOptions;

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
        .nest("/routes", routes::router())
        .nest("/users", users::router())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn index() -> &'static str {
    "Hello, World!"
}
