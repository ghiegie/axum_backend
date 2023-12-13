use axum::{extract::State, http::StatusCode, Json};
use odbc_api::{
    buffers::{BufferDesc, ColumnarAnyBuffer},
    ConnectionOptions, Cursor, Environment,
};
use std::sync::Arc;

use super::structs::CustomerData;

pub async fn get_customers(
    State((con_str, env)): State<(String, Arc<Environment>)>,
) -> (StatusCode, Json<Vec<CustomerData>>) {
    let env = Arc::clone(&env);
    let conn = env
        .connect_with_connection_string(&con_str, ConnectionOptions::default())
        .expect("ERROR: FAILED TO ESTABLISH DB CONNECTION");

    let buff_size = 100;
    let buff_desc = [BufferDesc::Text { max_str_len: 50 }; 9];
    let mut buff = ColumnarAnyBuffer::from_descs(buff_size, buff_desc);

    let mut init_val = 0;
    loop {
        // create sql query that fetches a specific rows of db
        let sql_query = format!("select * from CustomerTbl order by CURRENT_TIMESTAMP offset {} rows fetch next {} rows only", init_val, buff_size);

        // create cursor
        let cursor = conn
            .execute(&sql_query, ())
            .expect("ERROR: FAILED TO CREATE CURSOR")
            .expect("ERROR: CREATED CURSOR IS EMPTY");

        // create cursor
        let mut buff_cursor = cursor
            .bind_buffer(&mut buff)
            .expect("ERROR: FAILED TO CREATE BUFFER-CURSOR");

        if let Some(fetched_data) = buff_cursor.fetch().expect("ERROR: FAILED TO FETCH") { // if cursor contains data
            println!("cursor has data");
            let mut looping_vec = Vec::new();
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

            println!("{:?}", looping_vec);
        } else { // if cursor contains no data
            break;
        }

        init_val += buff_size;
    }

    // (StatusCode::NO_CONTENT, Json(Vec::new()))
    (StatusCode::OK, Json(Vec::new()))
    // "wait".to_owned()
}
