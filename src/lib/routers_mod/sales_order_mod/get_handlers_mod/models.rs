use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Customer {
    cust_id: String,
    name: String,
    addr: String,
    cont_pers: String,
    tin: String,
    tel_no: String,
    est: String,
    deliv_addr: String,
    email: String,
}
