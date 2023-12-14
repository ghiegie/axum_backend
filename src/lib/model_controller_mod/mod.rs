use odbc_api::Environment;
use std::sync::Arc;

use crate::error_mod::MyResult;

#[derive(Clone)]
pub struct ModelController {
    pub con_str: String,
    pub env: Arc<Environment>,
}

impl ModelController {
    pub async fn new(con_str: String, env: Arc<Environment>) -> MyResult<Self> {
        Ok(Self { con_str, env })
    }
}

pub mod sales_order_impl;
