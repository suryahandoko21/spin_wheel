use async_trait::async_trait;
use crate::{adapters::api::{spin_reward::spin_reward_payload::SpinRewardUpdatedPayload, shared::response::GenericResponse}, application::{repositories::spin_rewards_repository_abstract::SpinRewardEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils}, domain::error::ApiError};
pub struct UpdateSpinRewardsUseCase<'a>{
    post: &'a SpinRewardUpdatedPayload,
    repository: &'a dyn SpinRewardEntityAbstract
}

impl <'a>UpdateSpinRewardsUseCase<'a> {
    pub fn new(
            post: &'a SpinRewardUpdatedPayload,
            repository: &'a dyn SpinRewardEntityAbstract)->Self{
                UpdateSpinRewardsUseCase{post,repository}
            }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<GenericResponse> for UpdateSpinRewardsUseCase<'a>{
    async fn execute(&self) -> Result<GenericResponse, ApiError> {
        let spin_rewards = self.repository.update_spin_rewards(self.post).await;
        match spin_rewards {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Found Error", Some(e))),
        }
    } 
}
