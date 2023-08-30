use async_trait::async_trait;

use crate::{
    application::{repositories::spin_lists_repository_abstract::SpinListsEntityAbstract, usecases::{interfaces::AbstractUseCase}, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{ error::ApiError}, adapters::api::{spin_lists::spin_list_payloads::SpinListPayload, shared::response::GenericResponse},
};

pub struct PostSpinListsUseCase<'a>{
    post: &'a SpinListPayload,
    repository: &'a dyn SpinListsEntityAbstract
}

impl <'a>PostSpinListsUseCase<'a> {
    pub fn new(
            post: &'a SpinListPayload,
            repository: &'a dyn SpinListsEntityAbstract)->Self{
                PostSpinListsUseCase{post,repository}
            }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<GenericResponse> for PostSpinListsUseCase<'a>{
    async fn execute(&self) -> Result<GenericResponse, ApiError> {
        let spin_prizes = self.repository.post_one_spin_list(self.post).await;
        match spin_prizes {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Found Error", Some(e))),
        }
    } 
}
