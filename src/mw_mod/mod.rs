use axum::response::{IntoResponse, Response};

pub async fn main_res_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();

    "Test Response".into_response()
}
