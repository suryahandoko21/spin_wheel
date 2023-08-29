use async_trait::async_trait;
use crate::{application::{repositories::spin_rewards_repository_abstract::SpinRewardEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils}, domain::{spin_reward_entity::SpinRewardEntity, error::ApiError}};
pub struct ListSpinRewardsUseCase<'a> {
    company_code: &'a String,
    repository: &'a dyn SpinRewardEntityAbstract,
}

impl<'a> ListSpinRewardsUseCase<'a> {
    pub fn new(company_code: &'a String,
    repository: &'a dyn SpinRewardEntityAbstract) -> Self {    
        ListSpinRewardsUseCase { repository, company_code }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<Vec<SpinRewardEntity>> for ListSpinRewardsUseCase<'a> {
    async fn execute(&self) -> Result<Vec<SpinRewardEntity>, ApiError> {
        let company_uuid = self.company_code.clone();
        let spin_prizes = self.repository.get_all_spin_reward_by_company_code(company_uuid).await;
        match spin_prizes {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all DATA", Some(e))),
        }
    }
}
