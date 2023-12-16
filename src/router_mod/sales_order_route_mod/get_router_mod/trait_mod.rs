use axum::async_trait;
use odbc_api::Error;

use crate::error_mod::sales_order_error_mod::get_error::GetError;

#[async_trait]
pub trait SalesOrderGet {
    async fn get_customers(&self) -> Result<(), Error>;
    async fn get_items(&self) -> Result<(), GetError>;
    async fn get_color_coats(&self) -> Result<(), GetError>;
}
