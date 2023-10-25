use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpinRewardActiveEntity {
    pub status: bool,
    pub float_image : String,
    pub user_uuid: String,
    pub company_code: String,
    pub reward_list: Option<Vec<ActiveRewardEntity>>,
    pub chance_spin: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpinRewardEntity {
    pub reward_id: i32,
    pub reward_name: String,
    pub reward_note: String,
    pub reward_category: String,
    pub reward_amount: i32,
    pub reward_money: i32,
    pub reward_order: i32,
    pub companies_code: String,
    pub percentage: f64,
    pub reward_image: String,
    pub reward_status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
    pub updated_by: String,
}

impl SpinRewardEntity {
    pub fn new(
        reward_id: i32,
        reward_name: String,
        reward_note: String,
        reward_category: String,
        reward_amount: i32,
        reward_money: i32,
        reward_order: i32,
        companies_code: String,
        percentage: f64,
        reward_image: String,
        reward_status: String,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
        created_by: String,
        updated_by: String,
    ) -> Self {
        SpinRewardEntity {
            reward_id,
            reward_name,
            reward_note,
            reward_category,
            reward_amount,
            reward_money,
            reward_order,
            companies_code,
            percentage,
            reward_image,
            reward_status,
            created_at,
            updated_at,
            created_by,
            updated_by,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveRewardEntity {
    pub reward_id: i32,
    pub reward_name: String,
    pub reward_note: String,
    pub reward_category: String,
    pub reward_image: String,
    pub reward_status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl ActiveRewardEntity {
    pub fn new(
        reward_id: i32,
        reward_name: String,
        reward_note: String,
        reward_category: String,
        reward_image: String,
        reward_status: String,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    ) -> Self {
        ActiveRewardEntity {
            reward_id,
            reward_name,
            reward_note,
            reward_category,
            reward_image,
            reward_status,
            created_at,
            updated_at,
        }
    }
}
