use axum::{Extension, Json};
use axum::http::StatusCode;
use chrono::Utc;
use crate::utils::api_error::APIError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use sea_orm::TryGetError::DbErr;
use entity::post::ActiveModel;
use crate::models::post_models::{CreatePostModel, PostModel};

pub async fn create_post_POST(
    Extension(db):Extension<DatabaseConnection>,
    Extension(identity): Extension<entity::user::Model>,
    Json(post_data): Json<CreatePostModel>
) ->Result<(),APIError>{

    let post_entity: ActiveModel = entity::post::ActiveModel{
        title:Set(post_data.title),
        text:Set(post_data.text),
        image:Set(post_data.image),
        created_at:Set(Utc::now().naive_local()),
        user_id:Set(identity.id),
        ..Default::default()
    };

    post_entity.insert(&db)
        .await
        .map_err(|_| APIError {
            message:"Insert Failed".to_owned(),
            status_code:StatusCode::INTERNAL_SERVER_ERROR,
            error_code:Some(50)
        })?;

    Ok({})
}