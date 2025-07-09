use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;
use crate::models::user_models::{GetAllUsersModel, UpdateUserModel, };
use crate::utils::api_error::APIError;

pub async fn update_user_put(
    Extension(db):Extension<DatabaseConnection>,
    Path(uuid):Path<Uuid>,
    Json(user_data):Json<UpdateUserModel>,
)-> Result<String,APIError> {

    let mut user: entity::user::ActiveModel = entity::user::Entity::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(&db)
        .await
        .map_err(|err| APIError {
            message: format!("DB Error: {}", err),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
        .ok_or(APIError {
            message: "User not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(44),
        })?.into();

    user.name = Set(user_data.name);
    user.update(&db).await.map_err(|err|APIError{message:err.to_string()
        ,status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code:Some(50)})?;
    
    Ok("Successfully updated user".to_string())
}

pub async fn delete_user_delete(
    Extension(db):Extension<DatabaseConnection>,
    Path(uuid):Path<Uuid>,
)-> Result<(),APIError> {

    let user = entity::user::Entity::find()
        .filter(entity::user::Column::Uuid.eq(uuid))
        .one(&db)
        .await
        .map_err(|err| APIError {
            message: format!("DB Error: {}", err),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
        .ok_or(APIError {
            message: "User not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
            error_code: Some(44),
        })?;

    entity::user::Entity::delete_by_id(user.id).exec(&db).await
        .map_err(|err| APIError {
            message: format!("DB Error: {}", err),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?;

    Ok({})
}

pub async fn all_users_get( Extension(db):Extension<DatabaseConnection>,)-> Result<Json<Vec<GetAllUsersModel>>,APIError> {
    let users:Vec<GetAllUsersModel> = entity::user::Entity::find().all(&db).await
        .map_err(|err| APIError {
            message: format!("DB Error: {}", err),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            error_code: Some(50),
        })?
        .into_iter().map(|user| GetAllUsersModel{
        name:user.name,
        email:user.email,
        uuid:user.uuid,
        created_at:user.created_at
    }).collect();

    Ok(Json(users))
}