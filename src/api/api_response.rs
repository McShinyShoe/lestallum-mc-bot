use axum::{Json, http::StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse {
    pub code: u16,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl ApiResponse {
    pub fn ok(message: impl Into<String>) -> Self {
        Self {
            code: 200,
            message: message.into(),
            data: None,
        }
    }
    pub fn ok_data<D: Serialize>(message: impl Into<String>, data: D) -> Self {
        Self {
            code: 200,
            message: message.into(),
            data: Some(serde_json::to_value(data).unwrap()),
        }
    }

    pub fn error(message: impl Into<String>, code: u16) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }
    pub fn error_data<D: Serialize>(message: impl Into<String>, code: u16, data: D) -> Self {
        Self {
            code,
            message: message.into(),
            data: Some(serde_json::to_value(data).unwrap()),
        }
    }
    pub fn respond(status_code: StatusCode) -> Self {
        Self {
            code: status_code.as_u16(),
            message: format!("{}.", status_code.canonical_reason().unwrap_or("Error")),
            data: None,
        }
    }
    pub fn respond_data<D: Serialize>(status_code: StatusCode, data: D) -> Self {
        Self {
            code: status_code.as_u16(),
            message: format!("{}.", status_code.canonical_reason().unwrap_or("Error")),
            data: Some(serde_json::to_value(data).unwrap()),
        }
    }
}

pub type JsonApiResponse = Json<ApiResponse>;
pub type ResultApiResponse = Result<Json<ApiResponse>, (StatusCode, Json<ApiResponse>)>;
