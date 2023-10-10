use async_trait::async_trait;
use crate::{application::{repositories::spin_rewards_repository_abstract::SpinRewardEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils}, domain::{spin_reward_entity::SpinRewardActiveEntity, error::ApiError}};
pub struct ActiveSpinRewardsUseCase<'a> {
    company_code: &'a String,
    user_uuid: &'a String,
    is_login: & 'a bool, 
    repository: &'a dyn SpinRewardEntityAbstract,
}

impl<'a> ActiveSpinRewardsUseCase<'a> {
    pub fn new(
    company_code: &'a String,
    user_uuid: &'a String,
    is_login: & 'a bool, 
    repository: &'a dyn SpinRewardEntityAbstract) -> Self {    
        ActiveSpinRewardsUseCase { repository, company_code,user_uuid,is_login }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<SpinRewardActiveEntity> for ActiveSpinRewardsUseCase<'a> {
    async fn execute(&self) ->Result<SpinRewardActiveEntity, ApiError> {
        let spin_rewards = self.repository.get_active_spin_reward_by_company_code(self.company_code.to_string(),self.user_uuid.to_string(),*self.is_login).await;
        match spin_rewards {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all DATA", Some(e))),
        }
    }
}
