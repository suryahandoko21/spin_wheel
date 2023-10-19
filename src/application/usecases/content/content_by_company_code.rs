use crate::{
    application::{
        repositories::content_company_repository::ContentCompanyEntityAbstract,
        usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils,
    },
    domain::{content_entity::ContentEntity, error::ApiError},
};
use async_trait::async_trait;

pub struct ContentByCompannyCodeUseCase<'a> {
    company_code: &'a String,
    repository: &'a dyn ContentCompanyEntityAbstract,
}

impl<'a> ContentByCompannyCodeUseCase<'a> {
    pub fn new(company_code: &'a String, repository: &'a dyn ContentCompanyEntityAbstract) -> Self {
        ContentByCompannyCodeUseCase {
            repository,
            company_code,
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<ContentEntity> for ContentByCompannyCodeUseCase<'a> {
    async fn execute(&self) -> Result<ContentEntity, ApiError> {
        let spin_rewards = self
            .repository
            .get_content_by_company_by_id(self.company_code.to_string())
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
