use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub struct Exception<'a> {
    status_code: StatusCode,
    message: &'a str,
}

impl Exception<'static> {
    pub fn format(&self) -> impl IntoResponse {
        (self.status_code, Json(self.message))
    }
}
