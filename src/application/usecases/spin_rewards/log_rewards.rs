use crate::{
    application::{
        repositories::log_reward_repository::LogRewardAbstract,
        usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils,
    },
    domain::{error::ApiError, log_reward_entity::LogCustomRewardEntity},
};
use async_trait::async_trait;
pub struct LogRewardsUseCase<'a> {
    company_code: &'a String,
    repository: &'a dyn LogRewardAbstract,
}

impl<'a> LogRewardsUseCase<'a> {
    pub fn new(company_code: &'a String, repository: &'a dyn LogRewardAbstract) -> Self {
        LogRewardsUseCase {
            repository,
            company_code,
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<LogCustomRewardEntity> for LogRewardsUseCase<'a> {
    async fn execute(&self) -> Result<LogCustomRewardEntity, ApiError> {
        let spin_rewards = self
            .repository
            .get_log_reward_by_company_code(self.company_code.to_string())
            .await;
        match spin_rewards {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error(
                "Cannot get all DATA",
                Some(e),
            )),
        }
    }
}
