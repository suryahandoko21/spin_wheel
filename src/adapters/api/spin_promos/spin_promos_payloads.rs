use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct SpinPromosPayload {
    // implement for POST/UPDATE requests
    pub promo_amount: i32,
    pub user_id : String,
    pub username: String,
    pub point_currention_time : String,
    pub expired_at :String,
}
