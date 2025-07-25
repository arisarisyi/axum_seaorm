use axum::http::{header, StatusCode};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug)]
pub struct APIError {
    pub message: String,
    pub status_code: StatusCode,
    pub error_code: Option<i8>,
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        let status_code = self.status_code;
        (status_code,[(header::CONTENT_TYPE,"application/json")],
         Json(json!({"StatusCode":self.status_code.as_u16(),"ErrorCode":self.error_code,
             "Message":self.message  }))).into_response()
    }
}