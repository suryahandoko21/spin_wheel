use crate::{
    application::{repositories::spin_lists_repository_abstract::SpinListsEntityAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils},

};

pub struct DeleteOneSpinListsByIdUseCase<'a>{
    list_id: &'a i32,
    repository: &'a dyn SpinListsEntityAbstract
}

impl <'a>DeleteOneSpinListsByIdUseCase<'a> {
    pub fn new(
        list_id: &'a i32,
            repository: &'a dyn SpinListsEntityAbstract)->Self{
                DeleteOneSpinListsByIdUseCase{list_id,repository}
            }
}


// #[async_trait(?Send)]
// impl<'a> AbstractUseCase<GenericResponse> for DeleteOneSpinListsByIdUseCase<'a>{
//     async fn execute(&self) -> Result<GenericResponse, ApiError> {
//         let spin_lists = self.repository.delete_one_spin_list_by_id(*self.list_id).await;
//         match spin_lists {
//             Ok(facts) => Ok(facts),
//             Err(e) => Err(ErrorHandlingUtils::application_error("Found Error", Some(e))),
//         }
//     } 
// }




