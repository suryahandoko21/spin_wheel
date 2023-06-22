use async_trait::async_trait;

use crate::{
    application::{repositories::{spin_promos_repository_abstract::SpinPromosEntityAbstract}, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{spin_promos_entity::SpinPromosEntity, error::ApiError},
};

pub struct GetAllSpinPromosUseCase<'a> {
    repository: &'a dyn SpinPromosEntityAbstract,
}

impl<'a> GetAllSpinPromosUseCase<'a> {
    pub fn new(repository: &'a dyn SpinPromosEntityAbstract) -> Self {
        GetAllSpinPromosUseCase { repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<Vec<SpinPromosEntity>> for GetAllSpinPromosUseCase<'a> {
    async fn execute(&self) -> Result<Vec<SpinPromosEntity>, ApiError> {
        let spin_prizes = self.repository.get_all_spin_promos().await;
        match spin_prizes {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all DATA", Some(e))),
        }
    }
}

