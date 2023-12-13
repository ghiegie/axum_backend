use axum::Router;
use odbc_api::{
    sys::{AttrConnectionPooling, AttrCpMatch},
    Environment,
};
use std::sync::Arc;

use crate::routers_mod::sales_order_mod::sales_order_router;

pub fn give_state(con_str: String) -> (String, Arc<Environment>) {
    (
        con_str,
        Arc::new({
            unsafe {
                Environment::set_connection_pooling(AttrConnectionPooling::DriverAware)
                    .expect("UNSAFE CODE ERROR ");
            }

            let mut env = Environment::new().unwrap();

            env.set_connection_pooling_matching(AttrCpMatch::Strict)
                .expect("CREATION OF DB POOL FAILED");
            env
        }),
    )
}

pub fn create_main_router() -> Router {
    Router::new().nest("/sales_order", sales_order_router()).with_state(give_state("Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P\\MSSQLSERVER01;Database=TestDatabase;Trusted_Connection=yes;".to_owned()))
}

pub async fn start_server(ip_addr: &str, router: Router) {
    axum::serve(
        tokio::net::TcpListener::bind(ip_addr).await.unwrap(),
        router,
    )
    .await
    .expect("ERROR: AWAIT FOR SERVING RESULTED IN ERROR");
}
