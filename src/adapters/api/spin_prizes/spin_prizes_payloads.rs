use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SpinPrizesPayload {
    // implement for POST/UPDATE requests
    pub prize_weight: i32,
    pub prize_name: String,
    pub prize_note: String,
    pub prize_category: String,
    pub prize_amount: i32
}
