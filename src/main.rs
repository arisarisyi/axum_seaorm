use std::env;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, Set};
use dotenv::dotenv;
use uuid::Uuid;
use entity::user;



#[tokio::main]
async fn main() {
    server().await;
}

async fn server(){
dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let db: DatabaseConnection = Database::connect(&database_url).await.unwrap();

    let app: Router = Router::new()
        .route("/", get(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn test()-> impl IntoResponse {
    (StatusCode::ACCEPTED, "Hello, world!")
}

async fn create_user() -> impl IntoResponse {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let db: DatabaseConnection = Database::connect(&database_url).await.unwrap();

    let user_model = user::ActiveModel{
        name:Set("Alarisyi".to_owned()),
        email:Set("alarisyi@gmail.com".to_owned()),
        password:Set("12345678".to_owned()),
        uuid:Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    
    let _usr = user_model.insert(&db).await.unwrap();

    (StatusCode::ACCEPTED, "Inserted!")
}