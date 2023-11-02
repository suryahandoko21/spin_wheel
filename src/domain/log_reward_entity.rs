use chrono::NaiveDateTime;
use diesel::prelude::Queryable;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct LogRewardEntity {
    pub id: i32,
    pub companies_code: String,
    pub reward_before: String,
    pub reward_after: String,
    pub reward_change: String,
    pub remote_ip: String,
    pub action_change: String,
    pub created_at: NaiveDateTime,
    pub created_by: String,
}

impl LogRewardEntity {
    pub fn new(
        id: i32,
        companies_code: String,
        reward_before: String,
        reward_after: String,
        reward_change: String,
        remote_ip: String,
        action_change: String,
        created_at: NaiveDateTime,
        created_by: String,
    ) -> Self {
        LogRewardEntity {
            id,
            companies_code,
            reward_before,
            reward_after,
            reward_change,
            remote_ip,
            action_change,
            created_at,
            created_by,
        }
    }
}
