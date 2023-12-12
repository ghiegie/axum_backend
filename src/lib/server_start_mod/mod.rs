use axum::Router;

pub async fn start_server(ip_addr: &str, router: Router) {
    axum::serve(
        tokio::net::TcpListener::bind(ip_addr).await.unwrap(),
        router,
    )
    .await
    .expect("ERROR: AWAIT FOR SERVING RESULTED IN ERROR");
}

pub fn create_main_router() -> Router {
    Router::new().with_state()
}
