use async_trait::async_trait;
use odbc_api::{
    buffers::{BufferDesc, ColumnarAnyBuffer},
    ConnectionOptions, Cursor,
};

use super::ModelController;
use crate::{
    error_mod::*,
    routers_mod::sales_order_mod::get_handlers_mod::{models::Customer, traits::SalesOrderGet},
};

#[async_trait]
impl SalesOrderGet for ModelController {
    async fn sales_order_get_cust(&self) -> MyResult<Vec<Customer>> {
        // TODO: Handle unwrap here
        let conn = self
            .env
            .connect_with_connection_string(self.con_str.as_str(), ConnectionOptions::default())
            .unwrap();

        let buff_size = 100;
        let buff_desc = [BufferDesc::Text { max_str_len: 50 }; 9];
        let mut buff = ColumnarAnyBuffer::from_descs(buff_size, buff_desc);

        let mut init_val = 0;
        let mut looping_vec = Vec::new();
        loop {
            // create sql query that fetches a specific rows of db
            let sql_query = format!("select * from CustomerTbl order by CURRENT_TIMESTAMP offset {} rows fetch next {} rows only", init_val, buff_size);

            // create cursor
            let cursor = conn
                .execute(&sql_query, ())
                .expect("ERROR: FAILED TO CREATE CURSOR")
                .expect("ERROR: CREATED CURSOR IS EMPTY");

            // create buff w/ cursor
            let mut buff_cursor = cursor
                .bind_buffer(&mut buff)
                .expect("ERROR: FAILED TO CREATE BUFFER-CURSOR");

            if let Some(fetched_data) = buff_cursor.fetch().expect("ERROR: FAILED TO FETCH") {
                // if cursor contains data
                println!("cursor has data");
                for i in 0..fetched_data.num_cols() {
                    looping_vec.push(Vec::new());

                    let col = fetched_data.column(i);
                    if let Some(stream) = col.as_text_view() {
                        for optional_stream_slice in stream.iter() {
                            if let Some(stream_slice) = optional_stream_slice {
                                let str_result = String::from_utf8(stream_slice.to_vec()).unwrap(); // 500 error
                                looping_vec[i].push(str_result);
                            }
                        }
                    }
                }
            } else {
                // if cursor contains no data
                break;
            }

            init_val += buff_size;
        }

        println!("{:?}", looping_vec);

        // let mut list = Vec::new();
        // for (cust_id, name, addr, cont_pers, tin, tel_no, est, deliv_addr, email) in looping_vec[0].iter().zip(
        //     looping_vec[1].iter().zip(
        //         looping_vec[2].iter().zip(
        //             looping_vec[3].iter().zip(
        //                 looping_vec[4].iter().zip(
        //                     looping_vec[5].iter().zip(
        //                         looping_vec[6]
        //                             .iter()
        //                             .zip(looping_vec[7].iter().zip(looping_vec[8].iter())),
        //                     ),
        //                 ),
        //             ),
        //         ),
        //     ),
        // ) {
        //     list.push(DesigData::new(id, desig))
        // }

        // return (StatusCode::OK, Json(GetDesig::new(list)));

        Ok(Vec::new())
    }
}
