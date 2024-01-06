use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Serialize};
use serde_json::json;

/// 实现了IntoResponse的全局返回值
#[derive(Debug, Serialize)]
pub struct ResJson<T> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ResJson<T> {
    pub fn new(status: u16, message: &str, data: Option<T>) -> Self {
        ResJson {
            status,
            message: message.to_string(),
            data,
        }
    }

    pub fn success(data: T) -> Self {
        Self { status: 200, data: Some(data), message: "success".to_string()}
    }

    pub fn error(message: String) -> Self {
        Self { status: 400, data: None, message}
    }
}

impl<T: Serialize> IntoResponse for ResJson<T> {
    fn into_response(self) -> Response {
        let value = json!(self);
        Json(value).into_response()
    }
}
