use axum::body::Body;
use axum::http::Method;
use axum::middleware::from_fn;
use axum::Router;
use axum::routing::{delete, get, put};
use tower_http::cors::{Any, CorsLayer};
use crate::handlers::user_handlers::{all_users_get, delete_user_delete, update_user_put};
use crate::utils;

pub fn user_routes() ->Router{
    let cors = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    Router::new()
        .route("/api/user/{uuid}/update", put(update_user_put))
        .route("/api/users", get(all_users_get)).route_layer(from_fn(utils::guard::guard::<Body>))
        .route("/api/user/{uuid}",delete(delete_user_delete))
        .layer(cors)
}