use axum::routing::get;
use axum::Router;
use odbc_api::Environment;
use std::sync::Arc;

use self::handlers::{get_customers, test_get};

pub mod handlers;
pub mod structs;

pub fn get_router() -> Router<(String, Arc<Environment>)> {
    Router::new()
        .route("/customers", get(get_customers))
        .route("/test", get(test_get))
}
