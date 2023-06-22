use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use chrono::NaiveDateTime;
#[derive( Debug,Serialize,Deserialize)]
 pub struct SpinPromosPresenter {
    pub promo_amount : i32,
    pub promo_status : String,
    pub user_id : String,
    pub username: String,
    pub expired_at :NaiveDateTime,
    pub point_currention_time : NaiveDateTime,
    pub created_at : NaiveDateTime, 
    pub updated_at : NaiveDateTime,
    pub created_by : String,
    pub updated_by : String,
  
}
