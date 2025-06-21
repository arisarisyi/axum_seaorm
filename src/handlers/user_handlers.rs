use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;
use crate::models::user_models::{UpdateUserModel, UserModel};

pub async fn update_user_put(
    Extension(db):Extension<DatabaseConnection>,
    Path(uuid):Path<Uuid>,
    Json(user_data):Json<UpdateUserModel>,
)-> impl IntoResponse {

    let mut user: entity::user::ActiveModel = entity::user::Entity::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(&db)
        .await.unwrap().unwrap().into();

    user.name = Set(user_data.name);
    user.update(&db).await.unwrap();

    (StatusCode::ACCEPTED, "Updated!")
}

pub async fn delete_user_delete(
    Extension(db):Extension<DatabaseConnection>,
    Path(uuid):Path<Uuid>,
)-> impl IntoResponse {

    let user = entity::user::Entity::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(&db)
        .await.unwrap().unwrap();

    entity::user::Entity::delete_by_id(user.id).exec(&db).await.unwrap();

    (StatusCode::ACCEPTED, "Deleted!")
}

pub async fn all_users_get( Extension(db):Extension<DatabaseConnection>,)-> impl IntoResponse {
    let users:Vec<UserModel> = entity::user::Entity::find().all(&db).await.unwrap()
        .into_iter().map(|user| UserModel{
        name:user.name,
        password:user.password,
        email:user.email,
        uuid:user.uuid,
        created_at:user.created_at
    }).collect();

    (StatusCode::OK, Json(users))
}