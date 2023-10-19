use crate::{
    adapters::api::shared::response::GenericResponse,
    application::{
        repositories::cron_repository_abstract::CronEntityAbstract,
        usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils,
    },
    domain::error::ApiError,
};
use async_trait::async_trait;
pub struct PendingBeUseCase<'a> {
    repository: &'a dyn CronEntityAbstract,
}
impl<'a> PendingBeUseCase<'a> {
    pub fn new(repository: &'a dyn CronEntityAbstract) -> Self {
        PendingBeUseCase { repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<GenericResponse> for PendingBeUseCase<'a> {
    async fn execute(&self) -> Result<GenericResponse, ApiError> {
        let spin_rewards = self.repository.check_pending_post_be().await;
        match spin_rewards {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error(
                "Found Error",
                Some(e),
            )),
        }
    }
}
