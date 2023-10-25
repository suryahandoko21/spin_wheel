use async_trait::async_trait;

use crate::{
    adapters::api::{
        shared::response::SpinResponse, spin_useds::spin_tickets_payloads::SpinUsedPayload,
    },
    application::{
        repositories::spin_useds_repository_abstract::SpinUsedEntityAbstract,
        usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils,
    },
    domain::error::ApiError,
};

pub struct PostSpinUsedUseCase<'a> {
    post: &'a SpinUsedPayload,
    email: &'a String,
    company_code: &'a String,
    url_address: &'a String,
    remote_ip: &'a String,
    repository: &'a dyn SpinUsedEntityAbstract,
}

impl<'a> PostSpinUsedUseCase<'a> {
    pub fn new(
        post: &'a SpinUsedPayload,
        email: &'a String,
        company_code: &'a String,
        url_address: &'a String,
        remote_ip: &'a String,
        repository: &'a dyn SpinUsedEntityAbstract,
    ) -> Self {
        PostSpinUsedUseCase {
            post,
            repository,
            email,
            company_code,
            url_address,
            remote_ip,
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<SpinResponse> for PostSpinUsedUseCase<'a> {
    async fn execute(&self) -> Result<SpinResponse, ApiError> {
        let spin_used = self
            .repository
            .post_one_spin_useds(
                self.post,
                self.email.to_string(),
                self.company_code.to_string(),
                self.url_address.to_string(),
                self.remote_ip.to_string(),
            )
            .await;
        match spin_used {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error(
                "Found Error",
                Some(e),
            )),
        }
    }
}
