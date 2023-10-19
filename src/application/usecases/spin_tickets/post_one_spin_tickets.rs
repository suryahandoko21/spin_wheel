use async_trait::async_trait;

use crate::{
    adapters::api::{
        shared::response::TicketResponse, spin_tickets::spin_tickets_payloads::SpinTicketPayload,
    },
    application::{
        repositories::spin_ticket_repository_abstract::SpinTicketEntityAbstract,
        usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils,
    },
    domain::error::ApiError,
};

pub struct PostSpinTicketUseCase<'a> {
    company: &'a String,
    post: &'a SpinTicketPayload,
    repository: &'a dyn SpinTicketEntityAbstract,
}

impl<'a> PostSpinTicketUseCase<'a> {
    pub fn new(
        company: &'a String,
        post: &'a SpinTicketPayload,
        repository: &'a dyn SpinTicketEntityAbstract,
    ) -> Self {
        PostSpinTicketUseCase {
            company,
            post,
            repository,
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<TicketResponse> for PostSpinTicketUseCase<'a> {
    async fn execute(&self) -> Result<TicketResponse, ApiError> {
        let spin_tickets = self
            .repository
            .post_one_spin_tickets(self.company.to_string(), self.post)
            .await;
        match spin_tickets {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error(
                "Found Error",
                Some(e),
            )),
        }
    }
}
