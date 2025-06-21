use sea_orm::QueryFilter;
use sea_orm::ColumnTrait;
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum::body::Body;
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::utils::api_error::APIError;
use crate::utils::jwt::decode_jwt;

pub async fn guard<T>(mut req: Request<Body>, next: Next) ->Result<Response,APIError>{
    let token = req.headers()
        .get("Authorization")
        .ok_or(APIError {
            message: "Token missing".to_string(),
            status_code: StatusCode::BAD_REQUEST,
            error_code: Some(40),
        })?
        .to_str()
        .map_err(|_| APIError {
            message: "Invalid token encoding".to_string(),
            status_code: StatusCode::BAD_REQUEST,
            error_code: Some(40),
        })?
        .trim_start_matches("Bearer ")
        .to_string();

    let claims = decode_jwt(token).map_err(|_| APIError {
        message: "Unauthorized".to_owned(),
        status_code: StatusCode::UNAUTHORIZED,
        error_code: Some(41),
    })?;

    let db = req.extensions().get::<DatabaseConnection>()
        .ok_or(APIError { message: "Could not connect to database".to_owned(), status_code: StatusCode::INTERNAL_SERVER_ERROR, error_code: Some(50)  })?;

    let identity = entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(claims.claims.email.to_lowercase()))
        .one(db)
        .await.map_err(|err|  APIError { message: err.to_string(), status_code: StatusCode::INTERNAL_SERVER_ERROR, error_code:Some(50)})?
        .ok_or(APIError { message: "Unauthorized".to_owned(), status_code: StatusCode::UNAUTHORIZED, error_code: Some(41)  })?;

    req.extensions_mut().insert(identity);
    
    Ok(next.run(req).await)
}