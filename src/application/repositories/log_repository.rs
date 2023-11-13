use std::error::Error;

use async_trait::async_trait;
#[cfg(test)]
use mockall::{predicate::*, *};

use crate::domain::log_reward_entity::LogCustomEntity;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait LogAbstract {
    async fn log_actifity(
        &self,
        companies_code: String,
        created_by: String,
        before: String,
        after: String,
        change: String,
        entity_type: String,
        remote_ip: String,
        action_change: String,
    );
    async fn get_log_by_company_code(
        &self,
        company_code: String,
        etype: String,
    ) -> Result<Vec<LogCustomEntity>, Box<dyn Error>>;
}
