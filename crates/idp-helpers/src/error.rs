use axum::{http::StatusCode, response};
use serde::Serialize;

use crate::Json;

#[derive(Serialize)]
pub struct ErrorResponse {
    status: u16,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_id: Option<String>,
}

impl ErrorResponse {
    pub fn beans() -> Self {
        Self {
            status: 500,
            description: "You fucked it".to_string(),
            error_id: None,
        }
    }
}

pub enum HttpError {
    InternalServerError(ErrorResponse),
}

impl response::IntoResponse for HttpError {
    fn into_response(self) -> response::Response {
        match self {
            HttpError::InternalServerError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(e)).into_response()
            }
        }
    }
}
