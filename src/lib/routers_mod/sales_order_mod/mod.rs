use axum::Router;
use odbc_api::Environment;
use std::sync::Arc;

use self::get_handlers_mod::get_router;

pub mod get_handlers_mod;
pub mod post_handlers_mod;

pub fn sales_order_router() -> Router {
    Router::new().nest("/get", get_router())
}
