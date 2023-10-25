use crate::{
    adapters::api::{
        shared::response::GenericResponse, spin_reward::spin_reward_payload::SpinRewardPayload,
    },
    application::{
        repositories::spin_rewards_repository_abstract::SpinRewardEntityAbstract,
        usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils,
    },
    domain::error::ApiError,
};
use async_trait::async_trait;
pub struct PostSpinRewardsUseCase<'a> {
    email: &'a String,
    post: &'a SpinRewardPayload,
    repository: &'a dyn SpinRewardEntityAbstract,
}

impl<'a> PostSpinRewardsUseCase<'a> {
    pub fn new(
        email: &'a String,
        post: &'a SpinRewardPayload,
        repository: &'a dyn SpinRewardEntityAbstract,
    ) -> Self {
        PostSpinRewardsUseCase {
            email,
            post,
            repository,
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<GenericResponse> for PostSpinRewardsUseCase<'a> {
    async fn execute(&self) -> Result<GenericResponse, ApiError> {
        let spin_rewards = self
            .repository
            .post_spin_rewards(self.email.to_string(), self.post)
            .await;
        match spin_rewards {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error(
                "Found Error",
                Some(e),
            )),
        }
    }
}
