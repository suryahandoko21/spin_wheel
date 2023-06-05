use async_trait::async_trait;

use crate::{
    application::{repositories::spin_prizes_repository_abstract::SpinPrizesEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{spin_prizes_entity::SpinPrizesEntity, error::ApiError},
};

pub struct PostSpinPrizesUseCase<'a>{
    prizes_id: &'a i32,
    repository: &'a dyn SpinPrizesEntityAbstract
}

impl <'a>PostSpinPrizesUseCase<'a> {
    pub fn new(prizes_id: &'a i32,
            repository: &'a dyn SpinPrizesEntityAbstract)->Self{
                PostSpinPrizesUseCase{prizes_id,repository}
            }
}

#[async_trait(?Send)]
impl<'a>AbstractUseCase<SpinPrizesEntity> for PostSpinPrizesUseCase<'a>{
    async fn execute(&self)->Result<SpinPrizesEntity,ApiError>{
        let spin_prizes = self.repository.post_one_spin_prize(*self.prizes_id).await;
        match spin_prizes {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all DATA", Some(e))),
        }
    } 
}
