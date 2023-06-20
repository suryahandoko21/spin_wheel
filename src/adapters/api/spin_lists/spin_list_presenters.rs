use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::adapters::spi::prizes::models::SpinPrizes;

#[derive(Serialize, Deserialize, Debug)]
pub struct SpinListsPrizesPresenter {
    pub spin_prizes : SpinPrizes,
    pub company_code : String,
    pub list_status : String,
    pub quantity:i32,
    pub created_at: SystemTime,
    pub created_by: String,
    pub updated_at: SystemTime,
    pub updated_by :String,
}
