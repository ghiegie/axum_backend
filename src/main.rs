use axum_backend::server_start_mod::*;

#[tokio::main]
async fn main() {
    start_server("0.0.0.0:3000", create_main_router()).await
}
