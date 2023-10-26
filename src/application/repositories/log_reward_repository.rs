use async_trait::async_trait;
#[cfg(test)]
use mockall::{predicate::*, *};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait LogRewardAbstract {
    async fn log_reward_actifity(
        &self,
        companies_code: String,
        created_by: String,
        before: String,
        after: String,
    );
}
