use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SpinPrizesPresenter {
    pub prize_id: i32,
    pub prize_name: String,
    pub prize_note: String,
    pub prize_category: String,
    pub prize_amount: i32,
    pub prize_money: i32,
    pub percentage:i32
}
