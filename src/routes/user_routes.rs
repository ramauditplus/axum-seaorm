use axum::Router;
use axum::http::Method;
use axum::routing::{get, put, delete};
use tower_http::cors::{Any, CorsLayer};
use crate::handlers::user_handlers::{delete_user_delete, fetch_all_get, update_user_put};

pub fn user_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    let router =  Router::new()
        .route("/api/user/{uuid}/update", put(update_user_put))
        .route("/api/user/{uuid}/delete", delete(delete_user_delete))
        .route("/api/user/all", get(fetch_all_get))
        .layer(cors);

    router
}