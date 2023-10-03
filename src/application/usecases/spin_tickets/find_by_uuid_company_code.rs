use async_trait::async_trait;

use crate::{
    application::{repositories::spin_ticket_repository_abstract::SpinTicketEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::error::ApiError, adapters::api::shared::response::SpinAvailableResponse,
};

pub struct GetSpinTicketByUuidCompanyCodeUseCase<'a>{
    uuid: &'a String,
    company_code: &'a String,
    repository: &'a dyn SpinTicketEntityAbstract
}

impl <'a>GetSpinTicketByUuidCompanyCodeUseCase<'a> {
    pub fn new(
            uuid: &'a String,
            company_code: &'a String,
            repository: &'a dyn SpinTicketEntityAbstract)->Self{
                GetSpinTicketByUuidCompanyCodeUseCase{uuid,company_code,repository}
            }
}

#[async_trait(?Send)]
impl<'a>AbstractUseCase<SpinAvailableResponse> for GetSpinTicketByUuidCompanyCodeUseCase<'a>{
    async fn execute(&self)->Result<SpinAvailableResponse,ApiError>{
        let spin_ticket = self.repository.get_spin_ticket_by_uuid_company_code((*self.uuid.clone()).to_string(),self.company_code.clone().to_string()).await;
        match spin_ticket {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all DATA", Some(e))),
        }
    } 
}