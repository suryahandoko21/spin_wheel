use std::error::Error;

use async_trait::async_trait;
#[cfg(test)]
use mockall::{predicate::*, *};

use crate::domain::log_reward_entity::LogRewardEntity;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait LogRewardAbstract {
    async fn log_reward_actifity(
        &self,
        companies_code: String,
        created_by: String,
        before: String,
        after: String,
        change: String,
        remote_ip: String,
        action_change: String,
    );
    async fn get_log_reward_by_company_code(
        &self,
        company_code: String,
    ) -> Result<Vec<LogRewardEntity>, Box<dyn Error>>;
}
