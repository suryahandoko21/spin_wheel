use async_trait::async_trait;

use crate::{
    application::{repositories::spin_lists_repository_abstract::SpinListsEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{ error::ApiError, spin_lists_entity::SpinListsPrizesEntity},
};

pub struct GetOneSpinListsByIdUseCase<'a>{
    list_id: &'a i32,
    repository: &'a dyn SpinListsEntityAbstract
}

impl <'a>GetOneSpinListsByIdUseCase<'a> {
    pub fn new(
            list_id: &'a i32,
            repository: &'a dyn SpinListsEntityAbstract)->Self{
                GetOneSpinListsByIdUseCase{list_id,repository}
            }
}

#[async_trait(?Send)]
impl<'a>AbstractUseCase<SpinListsPrizesEntity> for GetOneSpinListsByIdUseCase<'a>{
    async fn execute(&self)->Result<SpinListsPrizesEntity,ApiError>{
        let spin_list_single = self.repository.get_one_spin_list_by_id(*self.list_id).await;
        match spin_list_single {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all DATA", Some(e))),
        }
    } 
}
