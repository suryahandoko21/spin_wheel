use async_trait::async_trait;

use crate::{
    application::{repositories::spin_lists_repository_abstract::SpinListsEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{ error::ApiError, spin_lists_entity::{SpinListsPrizesEntity}},
};


pub struct GetAllSpinListsUseCase<'a> {
    repository: &'a dyn SpinListsEntityAbstract,
}

impl<'a> GetAllSpinListsUseCase<'a> {
    pub fn new(repository: &'a dyn SpinListsEntityAbstract) -> Self {
        GetAllSpinListsUseCase { repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<Vec<SpinListsPrizesEntity>> for GetAllSpinListsUseCase<'a> {
    async fn execute(&self) -> Result<Vec<SpinListsPrizesEntity>, ApiError> {
        let spin_prizes = self.repository.get_all_spin_lists().await;
        match spin_prizes {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all DATA", Some(e))),
        }
    }
}
