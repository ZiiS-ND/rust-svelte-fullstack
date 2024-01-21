use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn create_route() -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
}

async fn get_user() -> &'static str {
    "Hi"
}

async fn create_user() {}

async fn update_user() {}

async fn delete_user() {}
