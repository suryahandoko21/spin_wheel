use crate::{
    application::{
        repositories::log_repository::LogAbstract, usecases::interfaces::AbstractUseCase,
        utils::error_handling_utils::ErrorHandlingUtils,
    },
    domain::{error::ApiError, log_reward_entity::LogCustomEntity},
};
use async_trait::async_trait;
pub struct LogRewardsUseCase<'a> {
    company_code: &'a String,
    etype: &'a String,
    repository: &'a dyn LogAbstract,
}

impl<'a> LogRewardsUseCase<'a> {
    pub fn new(
        company_code: &'a String,
        etype: &'a String,
        repository: &'a dyn LogAbstract,
    ) -> Self {
        LogRewardsUseCase {
            repository,
            company_code,
            etype,
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<Vec<LogCustomEntity>> for LogRewardsUseCase<'a> {
    async fn execute(&self) -> Result<Vec<LogCustomEntity>, ApiError> {
        let spin_rewards = self
            .repository
            .get_log_by_company_code(self.company_code.to_string(), self.etype.to_string())
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
