use async_trait::async_trait;
use crate::{adapters::api::{spin_reward::spin_reward_payload::SpinRewardPayload, shared::response::GenericResponse}, application::{repositories::spin_rewards_repository_abstract::SpinRewardEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils}, domain::error::ApiError};
pub struct PostSpinRewardsUseCase<'a>{
    post: &'a SpinRewardPayload,
    repository: &'a dyn SpinRewardEntityAbstract
}

impl <'a>PostSpinRewardsUseCase<'a> {
    pub fn new(
            post: &'a SpinRewardPayload,
            repository: &'a dyn SpinRewardEntityAbstract)->Self{
                PostSpinRewardsUseCase{post,repository}
            }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<GenericResponse> for PostSpinRewardsUseCase<'a>{
    async fn execute(&self) -> Result<GenericResponse, ApiError> {
        let spin_prizes = self.repository.post_spin_rewards(self.post).await;
        match spin_prizes {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Found Error", Some(e))),
        }
    } 
}
