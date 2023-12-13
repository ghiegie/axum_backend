use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type MyResult<T> = core::result::Result<T, MyError>;

#[derive(Debug)]
pub enum MyError {
    LoginFail,
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        println!("IntoResponse trait from MyError enum");

        (StatusCode::INTERNAL_SERVER_ERROR, "Unhandled Client Error").into_response()
    }
}
