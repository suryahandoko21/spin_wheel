use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct SpinRewardPayload {
    pub payload: Vec<SpinRewards>,
}
#[derive(Serialize, Deserialize, Debug,Clone)]
pub  struct SpinRewards {
    pub name: String,
    pub note: String,
    pub category: String,
    pub amount: i32,
    pub money: i32,
    pub percentage:i32,
    pub company_code:String,
    pub image: String,
    pub status:String,
    pub order:i32
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct SpinRewardUpdatedPayload {
    pub payload: Vec<SpinRewardUpdates>,
}
#[derive(Serialize, Deserialize, Debug,Clone)]
pub  struct SpinRewardUpdates {
    pub id : i32,
    pub name: String,
    pub note: String,
    pub category: String,
    pub amount: i32,
    pub money: i32,
    pub percentage:i32,
    pub company_code:String,
    pub image: String,
    pub status:String,
    pub order:i32,
}