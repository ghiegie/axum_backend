use axum::async_trait;
use odbc_api::{ConnectionOptions, Error, buffers::{ColumnarAnyBuffer, BufferDesc}, Cursor};
use std::sync::Arc;

use crate::{
    app_state_mod::mc_mod::ModelController, error_mod::sales_order_error_mod::get_error::GetError,
    router_mod::sales_order_route_mod::get_router_mod::trait_mod::*,
};

#[async_trait]
impl SalesOrderGet for ModelController {
    async fn get_customers(&self) -> Result<(), GetError> {
        let env = Arc::clone(&self.get_env());
        let conn =
            env.connect_with_connection_string(&self.get_con_str(), ConnectionOptions::default())?;

        let buff_size = 100;
        let mut buff =
            ColumnarAnyBuffer::from_descs(buff_size, [BufferDesc::Text { max_str_len: 50 }; 9]);

        let mut init_val = 0;
        let mut looping_vec = Vec::new();

        loop {
            let sql_query = format!("select * from CustomerTbl order by CURRENT_TIMESTAMP offset {} rows fetch next {} rows only", init_val, buff_size);
            let fetch = conn
                .execute(&sql_query, ())?
                .ok_or(GetError::NullErr)?
                .bind_buffer(&mut buff)?
                .fetch()?;

            if let Some(fetch_data) = fetch {
                for i in 0..fetch_data.num_cols() {
                    looping_vec.push(Vec::new());
                    let col = fetch_data.column(i);
                    let stream = col.as_text_view().ok_or(GetError::NullErr)?;
                }
            }
        }

        todo!()
    }

    async fn get_items(&self) -> Result<(), GetError> {
        todo!()
    }

    async fn get_color_coats(&self) -> Result<(), GetError> {
        todo!()
    }
}
