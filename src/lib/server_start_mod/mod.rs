use axum::{middleware, Router, extract::FromRef};
use odbc_api::{sys::{AttrConnectionPooling, AttrCpMatch}, Environment};
use std::sync::Arc;

use crate::{
    model_controller_mod::ModelController,
    routers_mod::{main_response_mapper, sales_order_mod::sales_order_router},
};

#[derive(Clone, FromRef)]
struct Appstate {
    mc: ModelController,
}

impl Appstate {
    fn new(mc: ModelController) -> Self {
        Self { mc }
    }
}

pub async fn create_main_router(mc: ModelController) -> Router {
    let mc = ModelController::new(
        "Driver={ODBC Driver 17 for SQL Server};Server=DESKTOP-DCDEB6P\\MSSQLSERVER01;Database=TestDatabase;Trusted_Connection=yes;".to_owned(), 
        Arc::new({
            unsafe {
                Environment::set_connection_pooling(AttrConnectionPooling::DriverAware).unwrap();
            }

            let mut env = Environment::new().unwrap();
            env.set_connection_pooling_matching(AttrCpMatch::Strict).unwrap();
            env
        })
    ).await.unwrap();

    Router::new()
        .nest("/sales_order", sales_order_router())
        .layer(middleware::map_response(main_response_mapper))
        .with_state(mc)
}

pub async fn start_server(ip_addr: &str, router: Router) {
    axum::serve(
        tokio::net::TcpListener::bind(ip_addr).await.unwrap(),
        router,
    )
    .await
    .expect("ERROR: AWAIT FOR SERVING RESULTED IN ERROR");
}
