use async_trait::async_trait;

use crate::{
    application::{repositories::spin_prizes_repository_abstract::SpinPrizesEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{spin_prizes_entity::SpinPrizesEntity, error::ApiError}, adapters::api::spin_prizes::spin_prizes_payloads::SpinPrizesPayload,
};

pub struct PostSpinPrizesUseCase<'a>{
    post: &'a SpinPrizesPayload,
    repository: &'a dyn SpinPrizesEntityAbstract
}

impl <'a>PostSpinPrizesUseCase<'a> {
    pub fn new(post: &'a SpinPrizesPayload,
            repository: &'a dyn SpinPrizesEntityAbstract)->Self{
                PostSpinPrizesUseCase{post,repository}
            }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<Vec<SpinPrizesEntity>> for PostSpinPrizesUseCase<'a>{
    async fn execute(&self) -> Result<Vec<SpinPrizesEntity>, ApiError> {
        let spin_prizes = self.repository.post_one_spin_prize(self.post).await;
        match spin_prizes {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all DATA", Some(e))),
        }
    } 
}
