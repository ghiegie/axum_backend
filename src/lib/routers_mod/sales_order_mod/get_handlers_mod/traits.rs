use async_trait::async_trait;

use super::models::Customer;
use crate::error_mod::MyResult;

#[async_trait]
pub trait SalesOrderGet {
    async fn sales_order_get_cust(&self) -> MyResult<Vec<Customer>>;
}
