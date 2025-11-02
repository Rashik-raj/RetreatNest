use axum::{
    body::Body, http::StatusCode, response::{IntoResponse, Response}, Json
};
use serde::Serialize;

#[derive(Serialize, Clone)]
struct ResponseData<T: Serialize> {
    data: T,
    message: String,
}

#[derive(Clone)]
pub struct CustomResponse<T: Serialize> {
    status_code: StatusCode,
    data: T,
    message: String,
}

impl<T: Serialize> IntoResponse for CustomResponse<T> {
    fn into_response(self) -> Response<Body> {
        let response_data = ResponseData {
            data: self.data,
            message: self.message,
        };
        return (self.status_code, Json(&response_data)).into_response();
    }
}

impl<T: Serialize> CustomResponse<T> {
    pub fn builder(data: T) -> CustomResponse<T> {
        return CustomResponse {
            status_code: StatusCode::OK,
            data: data,
            message: Default::default(),
        };
    }

    pub fn message(mut self, message: &str) -> Self{
        self.message = message.to_string();
        self
    }

    pub fn status_code(mut self, status_code: StatusCode) -> Self{
        self.status_code = status_code;
        self
    }

    pub fn build(&self) -> Response<Body> {
        let response = CustomResponse {
            status_code: self.status_code,
            data: &self.data,
            message: self.message.clone(),
        };
        return response.into_response();
    }
}

pub fn to_error_response<E: std::fmt::Display>(e: E, status: StatusCode) -> Response<Body> {
    CustomResponse::builder(())
        .message(&e.to_string())
        .status_code(status)
        .build()
}

pub fn to_error_response_with_message(message: &str, status: StatusCode) -> Response<Body> {
    CustomResponse::builder(())
        .message(message)
        .status_code(status)
        .build()
}
