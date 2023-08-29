use async_trait::async_trait;
use crate::{
    application::{repositories::spin_lists_repository_abstract::SpinListsEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::error::ApiError, adapters::api::{spin_lists::spin_list_payloads::SpinPostPayload, shared::response::GenericResponse},
};

pub struct PostSpinByUuidUseCase<'a>{
    post: &'a SpinPostPayload,
    repository: &'a dyn SpinListsEntityAbstract
}

impl <'a>PostSpinByUuidUseCase<'a> {
    pub fn new(
            post: &'a SpinPostPayload,
            repository: &'a dyn SpinListsEntityAbstract)->Self{
                PostSpinByUuidUseCase{post,repository}
            }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<GenericResponse> for PostSpinByUuidUseCase<'a>{
    async fn execute(&self) -> Result<GenericResponse, ApiError> {
        let spin_prizes = self.repository.post_spin_by_uuid(self.post).await;
        match spin_prizes {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Found Error", Some(e))),
        }
    } 
}
