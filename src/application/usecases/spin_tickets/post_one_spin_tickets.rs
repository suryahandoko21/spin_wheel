use async_trait::async_trait;

use crate::{
    application::{repositories::{spin_ticket_repository_abstract::SpinTicketEntityAbstract}, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{ error::ApiError}, adapters::api::{ shared::response::{GenericResponse, TicketResponse}, spin_tickets::spin_tickets_payloads::SpinTicketPayload},
};

pub struct PostSpinTicketUseCase<'a>{
    post: &'a SpinTicketPayload,
    repository: &'a dyn SpinTicketEntityAbstract
}

impl <'a>PostSpinTicketUseCase<'a> {
    pub fn new(
            post: &'a SpinTicketPayload,
            repository: &'a dyn SpinTicketEntityAbstract)->Self{
                PostSpinTicketUseCase{post,repository}
            }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<TicketResponse> for PostSpinTicketUseCase<'a>{
    async fn execute(&self) -> Result<TicketResponse, ApiError> {
        let spin_prizes = self.repository.post_one_spin_tickets(self.post).await;
        match spin_prizes {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Found Error", Some(e))),
        }
    } 
}
