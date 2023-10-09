use async_trait::async_trait;
use crate::{application::{repositories::spin_company_repository_abstract::SpinCompanyEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils}, domain::{spin_company_entity::SpinCompanyEntity, error::ApiError}};

pub struct CompaniesCodeUseCase<'a> {
    company_code: &'a String,
    repository: &'a dyn SpinCompanyEntityAbstract,
}

impl<'a> CompaniesCodeUseCase<'a> {
    pub fn new(
    company_code: &'a String,
    repository: &'a dyn SpinCompanyEntityAbstract) -> Self {    
        CompaniesCodeUseCase { repository, company_code }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<SpinCompanyEntity,> for CompaniesCodeUseCase<'a> {
    async fn execute(&self) -> Result<SpinCompanyEntity, ApiError> {
        let spin_rewards = self.repository.get_spin_company_by_code(self.company_code.to_string()).await;
        match spin_rewards {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all DATA", Some(e))),
        }
    }
}
