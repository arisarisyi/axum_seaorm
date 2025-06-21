use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Router};
use axum::routing::get;
use sea_orm::Database;

mod models;
mod routes;
mod handlers;
mod utils;

#[tokio::main]
async fn main() {
    server().await;
}

async fn server(){

    let conn_str = (*utils::constants::DATABASE_URL).clone();
    
    let db = Database::connect(conn_str).await.unwrap();
    
    let app: Router = Router::new()
        .route("/",get(test))
        .merge(routes::auth_routes::auth_routes())
        .merge(routes::user_routes::user_routes())
        .layer(Extension(db))
        ;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn test()-> impl IntoResponse {
    (StatusCode::ACCEPTED, "Hello, world!")
}