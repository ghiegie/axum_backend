use axum::{extract::State, http::StatusCode, Json};
use odbc_api::{
    buffers::{BufferDesc, ColumnarAnyBuffer},
    ConnectionOptions, Cursor, Environment,
};
use serde_json::{json, Value};
use std::sync::Arc;

// use super::structs::CustomerData;
use crate::{error_mod::*, model_controller_mod::ModelController};

use super::traits::SalesOrderGet;

pub async fn get_customers() {}
