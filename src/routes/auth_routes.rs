use axum::http::Method;
use axum::Router;
use axum::routing::{post};
use tower_http::cors::{Any, CorsLayer};
use crate::handlers::auth_handlers::{create_user_post, login_user_post};

pub fn auth_routes() ->Router{
    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(Any);
    
    Router::new()
    .route("/register", post(create_user_post))
        .route("/login", post(login_user_post))
        .layer(cors)
}