use axum::Router;
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};

use crate::routes;

pub async fn create_app() -> Router {
    Router::new()
        .nest_service(
            "/",
            ServeDir::new("static").fallback(ServeFile::new("/404.html")),
        )
        .merge(routes::user::create_route())
        .layer(CorsLayer::new().allow_origin(Any))
}
