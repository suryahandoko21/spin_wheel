use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct SpinPrizesPayload {
    // implement for POST/UPDATE requests

    pub prize_name: String,
    pub prize_note: String,
    pub prize_category: String,
    pub prize_amount: i32,
    pub percentage:i32
}
