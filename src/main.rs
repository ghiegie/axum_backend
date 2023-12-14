use axum_backend::{server_start_mod::*, model_controller_mod::ModelController};

#[tokio::main]
async fn main() {
    start_server("0.0.0.0:3000", create_main_router(ModelController::new(con_str, env).await)).await
}
