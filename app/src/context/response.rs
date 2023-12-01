use axum::{Json, http::StatusCode};
use serde_json::{json, Value};

type JsonBody<'a> = Json<Value>;
type JsonArrayBody<'a> = Json<Vec<&'a str>>;

fn empty_body<'a>() -> JsonBody<'a> {
    Json(json!({}))
}

fn empty_array_body<'a>() -> JsonArrayBody<'a> {
    Json(Vec::<&'a str>::new())
}

pub fn too_many_requests<'a>() -> (StatusCode, JsonBody<'a>) {
    (StatusCode::TOO_MANY_REQUESTS, empty_body())
}

pub fn payment_required<'a>() -> (StatusCode, JsonBody<'a>) {
    (StatusCode::PAYMENT_REQUIRED, empty_body())
}

pub fn internal_server_error<'a>() -> (StatusCode, JsonBody<'a>) {
    (StatusCode::INTERNAL_SERVER_ERROR, empty_body())
}

pub fn created<'a>() -> (StatusCode, JsonBody<'a>) {
    (StatusCode::CREATED, empty_body())
}
