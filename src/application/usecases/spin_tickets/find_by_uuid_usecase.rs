use async_trait::async_trait;

use crate::{
    application::{repositories::spin_ticket_repository_abstract::SpinTicketEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::error::ApiError, adapters::api::shared::response::SpinAvailableResponse,
};

pub struct GetSpinTicketByUuidUseCase<'a>{
    uuid: &'a String,
    repository: &'a dyn SpinTicketEntityAbstract
}

impl <'a>GetSpinTicketByUuidUseCase<'a> {
    pub fn new(
            uuid: &'a String,
            repository: &'a dyn SpinTicketEntityAbstract)->Self{
                GetSpinTicketByUuidUseCase{uuid,repository}
            }
}

#[async_trait(?Send)]
impl<'a>AbstractUseCase<SpinAvailableResponse> for GetSpinTicketByUuidUseCase<'a>{
    async fn execute(&self)->Result<SpinAvailableResponse,ApiError>{
        let spin_ticket = self.repository.get_spin_ticket_by_uuid(self.uuid.to_string()).await;
        match spin_ticket {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all DATA", Some(e))),
        }
    } 
}
