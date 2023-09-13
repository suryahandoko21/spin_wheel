use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SpinRewardsPresenter {
    pub reward_id: i32,
    pub reward_name: String,
    pub reward_note: String,
    pub reward_category: String,
    pub reward_amount: i32,
    pub reward_money: i32,
    pub percentage:i32,
    pub reward_image:String,
    pub reward_status:String,
    pub reward_order :i32
}
