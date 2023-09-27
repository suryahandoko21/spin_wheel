use async_trait::async_trait;
use crate::{application::{repositories::spin_rewards_repository_abstract::SpinRewardEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils}, domain::{spin_reward_entity::SpinRewardActiveEntity, error::ApiError}};
pub struct ActiveSpinRewardsUseCase<'a> {
    company_code: &'a String,
    user_uuid: &'a String,
    repository: &'a dyn SpinRewardEntityAbstract,
}

impl<'a> ActiveSpinRewardsUseCase<'a> {
    pub fn new(
    company_code: &'a String,
    user_uuid: &'a String,
    repository: &'a dyn SpinRewardEntityAbstract) -> Self {    
        ActiveSpinRewardsUseCase { repository, company_code,user_uuid }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<SpinRewardActiveEntity> for ActiveSpinRewardsUseCase<'a> {
    async fn execute(&self) ->Result<SpinRewardActiveEntity, ApiError> {
        let company_uuid = self.company_code.clone();
        let uuid = self.user_uuid.clone();
        let spin_rewards = self.repository.get_active_spin_reward_by_company_code(company_uuid,uuid).await;
        match spin_rewards {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all DATA", Some(e))),
        }
    }
}
