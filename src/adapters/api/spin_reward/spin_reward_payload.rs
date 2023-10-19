use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Clone,ToSchema)]
pub struct SpinRewardActivePayload {
    pub user_uuid:String
 }

use utoipa::ToSchema;
#[derive(Serialize, Deserialize, Debug,Clone,ToSchema)]
pub struct SpinRewardPayload {
    pub company_code:String,
    pub detail: Vec<SpinRewards>,
 }
#[derive(Serialize, Deserialize, Debug,Clone,ToSchema)]
pub  struct SpinRewards {
    pub name: String,
    pub desc: String,
    pub category: String,
    pub amount: i32,
    pub money: i32,
    pub percentage:f64,
    pub image: String,
    pub status:String,
    pub order:i32
}

#[derive(Serialize, Deserialize, Debug,Clone,ToSchema)]
pub struct SpinRewardUpdatedPayload {
    pub company_code:String,
    pub detail: Vec<SpinRewardUpdates>,
}
#[derive(Serialize, Deserialize, Debug,Clone,ToSchema)]
pub  struct SpinRewardUpdates {
    pub id : i32,
    pub name: String,
    pub desc: String,
    pub category: String,
    pub amount: i32,
    pub money: i32,
    pub percentage:f64,
    pub image: String,
    pub status:String,
    pub order:i32,
}