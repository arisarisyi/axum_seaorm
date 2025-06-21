use sea_orm::{ActiveModelTrait, QueryFilter};
use sea_orm::ColumnTrait;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use sea_orm::{ DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;
use entity::user;
use migration::Condition;
use crate::models::user_models::{CreateUserModel, LoginUserModel, UserModel};

pub async fn create_user_post(
    Extension(db):Extension<DatabaseConnection>,
    Json(user_data): Json<CreateUserModel>,
) -> impl IntoResponse {
    let password = hash(&user_data.password,DEFAULT_COST)
        .expect("Password hash invalid");

    let user_model = user::ActiveModel{
        name:Set(user_data.name.to_owned()),
        email:Set(user_data.email.to_owned()),
        password:Set(password.to_owned()),
        uuid:Set(Uuid::new_v4()),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    user_model.insert(&db).await.unwrap();

    (StatusCode::ACCEPTED, "Inserted!")
}

pub async fn login_user_post(
    Extension(db):Extension<DatabaseConnection>,
    Json(user_data): Json<LoginUserModel>,
) -> impl IntoResponse {

    let find_user = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq(user_data.email))
        ).one(&db)
        .await.unwrap().unwrap();


    let _password_validated = verify(&user_data.password,&find_user.password).unwrap();
    
    let data = UserModel{
        name: find_user.name,
        email:find_user.email,
        password:find_user.password,
        uuid:find_user.uuid,
        created_at:find_user.created_at
    };

    (StatusCode::ACCEPTED, Json(data))
}