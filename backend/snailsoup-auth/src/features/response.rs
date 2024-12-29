use axum::response::IntoResponse;

pub struct HttpError {
    error_code: axum::http::StatusCode,
    message: Option<String>,
}

impl From<axum::http::StatusCode> for HttpError {
    fn from(value: axum::http::StatusCode) -> Self {
        HttpError {
            error_code: value,
            message: None,
        }
    }
}

impl From<(axum::http::StatusCode, &str)> for HttpError {
    fn from(value: (axum::http::StatusCode, &str)) -> Self {
        HttpError {
            error_code: value.0,
            message: Some(value.1.to_owned()),
        }
    }
}

impl From<&str> for HttpError {
    fn from(value: &str) -> Self {
        HttpError {
            error_code: axum::http::StatusCode::BAD_REQUEST,
            message: Some(value.to_owned()),
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        match self.message {
            Some(message) => (self.error_code, message).into_response(),
            None => self.error_code.into_response(),
        }
    }
}
