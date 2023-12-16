use odbc_api::{
    sys::{AttrConnectionPooling, AttrCpMatch},
    Environment, Error,
};
use std::sync::Arc;

pub mod sales_order_impl;

#[derive(Clone, Debug)]
pub struct ModelController {
    con_str: String,
    env: Arc<Environment>,
}

impl ModelController {
    pub fn new(con_str: String) -> Result<Self, Error> {
        Ok(Self {
            con_str,
            env: Arc::new({
                unsafe {
                    Environment::set_connection_pooling(AttrConnectionPooling::DriverAware)?;
                }

                let mut env = Environment::new().unwrap();
                env.set_connection_pooling_matching(AttrCpMatch::Strict)?;
                env
            }),
        })
    }
}

impl ModelController {
    fn get_con_str(&self) -> String {
        self.con_str.clone()
    }

    fn get_env(&self) -> Arc<Environment> {
        Arc::clone(&self.env)
    }
}
