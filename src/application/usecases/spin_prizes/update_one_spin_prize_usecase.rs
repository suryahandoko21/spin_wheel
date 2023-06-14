use async_trait::async_trait;

use crate::{
    application::{repositories::spin_prizes_repository_abstract::SpinPrizesEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{ error::ApiError}, adapters::api::{spin_prizes::spin_prizes_payloads::SpinPrizesPayload, shared::response::GenericResponse},
};

pub struct UpdateSpinPrizesUseCase<'a>{
    prizes_id: &'a i32,
    post: &'a SpinPrizesPayload,
    repository: &'a dyn SpinPrizesEntityAbstract
}

impl <'a>UpdateSpinPrizesUseCase<'a> {
    pub fn new(
            prizes_id: &'a i32,
            post: &'a SpinPrizesPayload,
            repository: &'a dyn SpinPrizesEntityAbstract)->Self{
                UpdateSpinPrizesUseCase{prizes_id,post,repository}
            }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<GenericResponse> for UpdateSpinPrizesUseCase<'a>{
    async fn execute(&self) -> Result<GenericResponse, ApiError> {
        let spin_prizes = self.repository.updated_one_spin_prize(*self.prizes_id,self.post).await;
        match spin_prizes {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Found Error", Some(e))),
        }
    } 
}
