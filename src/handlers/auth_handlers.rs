use sea_orm::{ActiveModelTrait, QueryFilter};
use sea_orm::ColumnTrait;
use axum::http::StatusCode;
use axum::{Extension, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use sea_orm::{ DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;
use entity::user;
use migration::Condition;
use crate::models::user_models::{CreateUserModel, LoginResponseModel, LoginUserModel,};
use crate::utils::api_error::APIError;
use crate::utils::jwt::encode_jwt;

pub async fn create_user_post(
    Extension(db):Extension<DatabaseConnection>,
    Json(user_data): Json<CreateUserModel>,
) -> Result<(),APIError> {
    
    let find_user = entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(user_data.email.clone())).one(&db)
        .await
        .map_err(|err|APIError{message:err.to_string()
            ,status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code:Some(50)})?;
    
    if find_user != None{
        return Err(APIError{message:"User exists".to_string(), status_code:StatusCode::BAD_REQUEST
            , error_code:Some(40)})
    }
    
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

    user_model.insert(&db).await
        .map_err(|err|APIError{message:err.to_string()
            ,status_code:StatusCode::INTERNAL_SERVER_ERROR, error_code:Some(50)})?;

   Ok({})
}

pub async fn login_user_post(
    Extension(db): Extension<DatabaseConnection>,
    Json(user_data): Json<LoginUserModel>,
) -> Result<Json<LoginResponseModel>,APIError> {
    let user = user::Entity::find()
        .filter(Condition::all().add(user::Column::Email.eq(user_data.email.clone())))
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

    let is_valid = verify(&user_data.password, &user.password).map_err(|err| APIError {
        message: format!("Hash verification failed: {}", err),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: Some(50),
    })?;

    if !is_valid {
        return Err(APIError {
            message: "Incorrect password".to_string(),
            status_code: StatusCode::BAD_REQUEST,
            error_code: Some(40),
        });
    }
    
    let token = encode_jwt(user_data.email)
        .map_err(|_|APIError{message:"Failed to login".to_owned(),status_code:StatusCode::UNAUTHORIZED,error_code:Some(41)})?;

    let response = LoginResponseModel {
     token
    };

    Ok(Json(response))
}