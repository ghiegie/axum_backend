use axum::extract::FromRef;

use self::mc_mod::ModelController;

pub mod mc_mod;

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    mc: ModelController,
}

impl AppState {
    pub fn new(mc: ModelController) -> Self {
        Self { mc }
    }
}

impl AppState {
    pub fn get_mc(&self) -> &ModelController {
        &self.mc
    }
}
