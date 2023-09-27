use async_trait::async_trait;

use crate::{
    application::{repositories::spin_ticket_repository_abstract::SpinTicketEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::error::ApiError, adapters::api::shared::response::SpinAvailableResponse,
};

pub struct GetSpinTicketByUseridUseCase<'a>{
    userid: &'a String,
    repository: &'a dyn SpinTicketEntityAbstract
}

impl <'a>GetSpinTicketByUseridUseCase<'a> {
    pub fn new(
        userid: &'a String,
        repository: &'a dyn SpinTicketEntityAbstract)->Self{
            GetSpinTicketByUseridUseCase{userid,repository}
            }
}

#[async_trait(?Send)]
impl<'a>AbstractUseCase<SpinAvailableResponse> for GetSpinTicketByUseridUseCase<'a>{
    async fn execute(&self)->Result<SpinAvailableResponse,ApiError>{
        let spin_ticket = self.repository.get_spin_ticket_by_uuid((*self.userid.clone()).to_string()).await;
        match spin_ticket {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all DATA", Some(e))),
        }
    } 
}
