use axum::response::Response;

pub mod sales_order_mod;

pub async fn main_response_mapper(res: Response) -> Response {
    println!("PASSING THROUGH THE MAIN RESPONSE MAPPER");

    res
}