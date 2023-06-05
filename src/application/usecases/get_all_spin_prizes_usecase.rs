use async_trait::async_trait;

use crate::{
    application::{repositories::spin_prizes_repository_abstract::SpinPrizesEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},
    domain::{spin_prizes_entity::SpinPrizesEntity, error::ApiError},
};

pub struct GetAllSpinPrizesUseCase<'a> {
    repository: &'a dyn SpinPrizesEntityAbstract,
}

impl<'a> GetAllSpinPrizesUseCase<'a> {
    pub fn new(repository: &'a dyn SpinPrizesEntityAbstract) -> Self {
        GetAllSpinPrizesUseCase { repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<Vec<SpinPrizesEntity>> for GetAllSpinPrizesUseCase<'a> {
    async fn execute(&self) -> Result<Vec<SpinPrizesEntity>, ApiError> {
        let spin_prizes = self.repository.get_all_spin_prizes().await;

        match spin_prizes {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot get all DATA", Some(e))),
        }
    }
}



// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::io::{Error, ErrorKind};

//     use crate::{application::repositories::dog_facts_repository_abstract::MockDogFactsRepositoryAbstract, domain::dog_fact_entity::DogFactEntity};

//     #[actix_rt::test]
//     async fn test_should_return_error_with_generic_message_when_unexpected_repo_error() {
//         // given the "all dog facts" usecase repo with an unexpected random error
//         let mut dog_fact_repository = MockDogFactsRepositoryAbstract::new();
//         dog_fact_repository
//             .expect_get_all_dog_facts()
//             .with()
//             .times(1)
//             .returning(|| Err(Box::new(Error::new(ErrorKind::Other, "oh no!"))));

//         // when calling usecase
//         let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&dog_fact_repository);
//         let data = get_all_dog_facts_usecase.execute().await;

//         // then exception
//         assert!(data.is_err());
//         let result = data.unwrap_err();
//         assert_eq!("Cannot get all dog facts", result.message);
//     }

//     #[actix_rt::test]
//     async fn test_should_return_empty_list() {
//         // given the "all dog facts" usecase repo returning an empty list
//         let mut dog_fact_repository = MockDogFactsRepositoryAbstract::new();
//         dog_fact_repository.expect_get_all_dog_facts().with().times(1).returning(|| Ok(Vec::<DogFactEntity>::new()));

//         // when calling usecase
//         let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&dog_fact_repository);
//         let data = get_all_dog_facts_usecase.execute().await.unwrap();

//         // then assert the result is an empty list
//         assert_eq!(data.len(), 0);
//     }

//     #[actix_rt::test]
//     async fn test_should_return_list() {
//         // given the "all dog facts" usecase repo returning a list of 2 entities
//         let mut dog_fact_repository = MockDogFactsRepositoryAbstract::new();
//         dog_fact_repository.expect_get_all_dog_facts().with().times(1).returning(|| {
//             Ok(vec![
//                 DogFactEntity {
//                     fact_id: 1,
//                     fact: String::from("fact1"),
//                 },
//                 DogFactEntity {
//                     fact_id: 2,
//                     fact: String::from("fact2"),
//                 },
//             ])
//         });

//         // when calling usecase
//         let get_all_dog_facts_usecase = GetAllDogFactsUseCase::new(&dog_fact_repository);
//         let data = get_all_dog_facts_usecase.execute().await.unwrap();

//         // then assert the result is an empty list
//         assert_eq!(data.len(), 2);
//     }
// }
