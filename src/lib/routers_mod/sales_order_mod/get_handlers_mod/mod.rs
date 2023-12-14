use axum::Router;
use odbc_api::Environment;
use std::sync::Arc;

pub mod handlers;
pub mod models;
pub mod traits;

pub fn get_router() -> Router {
    Router::new()
}
