pub mod config;
pub mod routes;
pub mod users;

pub struct AppState {
    pub db: sqlx::PgPool,
}

