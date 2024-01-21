use axum::{http::HeaderValue, Router};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

use crate::routes;

pub async fn create_app() -> Router {
    let is_production = std::env::var("PRODUCTION");

    let origin = match is_production {
        Ok(..) => "https://productionurl.com".parse::<HeaderValue>().unwrap(),
        Err(..) => "*".parse::<HeaderValue>().unwrap(),
    };

    Router::new()
        .nest_service(
            "/",
            ServeDir::new("static").fallback(ServeFile::new("/404.html")),
        )
        .merge(routes::user::create_route())
        .layer(CorsLayer::new().allow_origin(origin))
}
