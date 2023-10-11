use async_trait::async_trait;
use crate::{application::{repositories::content_company_repository::ContentCompanyEntityAbstract, utils::error_handling_utils::ErrorHandlingUtils, usecases::interfaces::AbstractUseCase}, domain::{content_entity::ContentEntity, error::ApiError}};

pub struct ContentDefaultUseCase<'a> {
    repository: &'a dyn ContentCompanyEntityAbstract,
}

impl<'a> ContentDefaultUseCase<'a> {
    pub fn new(
    repository: &'a dyn ContentCompanyEntityAbstract) -> Self {    
        ContentDefaultUseCase { repository }
    }
}


#[async_trait(?Send)]
impl<'a> AbstractUseCase<ContentEntity,> for ContentDefaultUseCase<'a> {
    async fn execute(&self) -> Result<ContentEntity, ApiError> {
        let spin_rewards = self.repository.get_content_default().await;
        match spin_rewards {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all DATA", Some(e))),
        }
    }
}
