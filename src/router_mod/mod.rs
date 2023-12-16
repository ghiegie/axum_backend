use axum::{middleware, Router};
use odbc_api::Error;

use crate::{
    app_state_mod::{mc_mod::ModelController, AppState},
    const_mod::CON_STR,
    mw_mod::main_res_mapper,
};

pub mod sales_order_route_mod;

pub fn main_router() -> Result<Router, Error> {
    Ok(Router::new()
        .with_state(AppState::new(ModelController::new(String::from(CON_STR))?))
        .layer(middleware::map_response(main_res_mapper)))
}
