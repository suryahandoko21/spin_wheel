use async_trait::async_trait;

use crate::{
    application::{repositories::spin_promos_repository_abstract::SpinPromosEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{ error::ApiError, spin_promos_entity::SpinPromosEntity}, adapters::api::shared::response::GenericResponse,
};

pub struct PostSpinPromosUseCase<'a>{
    post: &'a SpinPromosEntity,
    repository: &'a dyn SpinPromosEntityAbstract
}

impl <'a>PostSpinPromosUseCase<'a> {
    pub fn new(
            post: &'a SpinPromosEntity,
            repository: &'a dyn SpinPromosEntityAbstract)->Self{
                PostSpinPromosUseCase{post,repository}
            }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<GenericResponse> for PostSpinPromosUseCase<'a>{
    async fn execute(&self) -> Result<GenericResponse, ApiError> {
        let spin_prizes = self.repository.post_one_spin_promos(self.post).await;
        match spin_prizes {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Found Error", Some(e))),
        }
    } 
}
