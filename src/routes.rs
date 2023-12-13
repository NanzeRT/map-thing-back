use std::sync::Arc;

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

use crate::{AppState, users::User};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Route {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub author_id: i32,
    pub stars: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RouteWithAuthor {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub author: User,
    pub stars: i32,
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_routes).post(add_route))
        .route("/:id", get(get_routes_id))
}

pub async fn get_routes(State(app_state): State<Arc<AppState>>) -> Json<Vec<RouteWithAuthor>> {
    let routes = sqlx::query_as!(Route, r#"SELECT * FROM routes"#,)
        .fetch_all(&app_state.db)
        .await
        .unwrap();

    let mut routes_with_author = Vec::new();

    for route in routes {
        let author = sqlx::query_as!(User, r#"SELECT * FROM users WHERE id = $1"#, route.author_id)
            .fetch_one(&app_state.db)
            .await
            .unwrap();

        routes_with_author.push(RouteWithAuthor {
            id: route.id,
            name: route.name,
            description: route.description,
            author,
            stars: route.stars,
        });
    }

    routes_with_author.into()

}

pub async fn get_routes_id(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<RouteWithAuthor>, ()> {
    let route = sqlx::query_as!(Route, r#"SELECT * FROM routes WHERE id = $1"#, id)
        .fetch_one(&app_state.db)
        .await.map_err(|_| ())?;

    let author = sqlx::query_as!(User, r#"SELECT * FROM users WHERE id = $1"#, route.author_id)
        .fetch_one(&app_state.db)
        .await.map_err(|_| ())?;

    Ok(RouteWithAuthor {
        id: route.id,
        name: route.name,
        description: route.description,
        author,
        stars: route.stars,
    }.into())
}

pub async fn add_route(
    State(app_state): State<Arc<AppState>>,
    Json(route): Json<Route>,
) -> Json<Route> {
    let route = sqlx::query_as!(
        Route,
        r#"
        INSERT INTO routes (name, description, author_id, stars)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        route.name,
        route.description,
        route.author_id,
        route.stars
    )
    .fetch_one(&app_state.db)
    .await
    .unwrap();

    route.into()
}
