use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct SpinUsedPayload {
    // implement for POST/UPDATE requests
    pub user_uuid: String,
    pub company_code :String
}
