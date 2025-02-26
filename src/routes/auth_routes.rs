use axum::Router;
use axum::http::Method;
use axum::routing::post;
use tower_http::cors::{Any, CorsLayer};
use crate::handlers::auth_handlers::{create_user_post, login_user_post};

pub fn auth_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(Any);

    let router =  Router::new()
        .route("/api/user/register", post(create_user_post))
        .route("/api/user/login", post(login_user_post))
        .layer(cors);
    
    router
}
