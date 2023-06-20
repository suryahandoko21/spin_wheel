use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct SpinListPayload {
    // implement for POST/UPDATE requests
    pub company_code: String,
    pub list_status: String,
    pub quantity: i32,
    pub spin_prizes_id: i32,
}
