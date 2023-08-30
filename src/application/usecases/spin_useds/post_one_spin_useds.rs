use async_trait::async_trait;

use crate::{
    application::{repositories::spin_useds_repository_abstract::SpinUsedEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::error::ApiError, adapters::api::{ shared::response::SpinResponse, spin_useds::spin_tickets_payloads::SpinUsedPayload},
};

pub struct PostSpinUsedUseCase<'a>{
    post: &'a SpinUsedPayload,
    repository: &'a dyn SpinUsedEntityAbstract
}

impl <'a>PostSpinUsedUseCase<'a> {
    pub fn new(
            post: &'a SpinUsedPayload,
            repository: &'a dyn SpinUsedEntityAbstract)->Self{
                PostSpinUsedUseCase{post,repository}
            }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<SpinResponse> for PostSpinUsedUseCase<'a>{
    async fn execute(&self) -> Result<SpinResponse, ApiError> {
        let spin_prizes = self.repository.post_one_spin_useds(self.post).await;
        match spin_prizes {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Found Error", Some(e))),
        }
    } 
}
