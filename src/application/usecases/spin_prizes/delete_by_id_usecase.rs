use async_trait::async_trait;

use crate::{
    application::{repositories::spin_prizes_repository_abstract::SpinPrizesEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{error::ApiError}, adapters::api::shared::response::GenericResponse,
};

pub struct DeleteOneSpinPrizesByIdUseCase<'a>{
    prize_id: &'a i32,
    repository: &'a dyn SpinPrizesEntityAbstract
}

impl <'a>DeleteOneSpinPrizesByIdUseCase<'a> {
    pub fn new(
            prize_id: &'a i32,
            repository: &'a dyn SpinPrizesEntityAbstract)->Self{
                DeleteOneSpinPrizesByIdUseCase{prize_id,repository}
            }
}


#[async_trait(?Send)]
impl<'a> AbstractUseCase<GenericResponse> for DeleteOneSpinPrizesByIdUseCase<'a>{
    async fn execute(&self) -> Result<GenericResponse, ApiError> {
        let spin_prizes = self.repository.delete_one_spin_prize_by_id(*self.prize_id).await;
        match spin_prizes {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Found Error", Some(e))),
        }
    } 
}




