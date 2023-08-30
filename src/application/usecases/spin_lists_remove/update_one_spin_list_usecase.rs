

use crate::{
    application::repositories::spin_lists_repository_abstract_remove::SpinListsEntityAbstract, adapters::api::spin_lists_remove::spin_list_payloads::SpinListPayload,
};

pub struct UpdateSpinListsUseCase<'a>{
    list_id: &'a i32,
    post: &'a SpinListPayload,
    repository: &'a dyn SpinListsEntityAbstract
}

impl <'a>UpdateSpinListsUseCase<'a> {
    pub fn new(
        list_id: &'a i32,
            post: &'a SpinListPayload,
            repository: &'a dyn SpinListsEntityAbstract)->Self{
                UpdateSpinListsUseCase{list_id,post,repository}
            }
}

// #[async_trait(?Send)]
// impl<'a> AbstractUseCase<GenericResponse> for UpdateSpinListsUseCase<'a>{
//     async fn execute(&self) -> Result<GenericResponse, ApiError> {
//         let spin_prizes = self.repository.updated_one_spin_list(*self.list_id,self.post).await;
//         match spin_prizes {
//             Ok(facts) => Ok(facts),
//             Err(e) => Err(ErrorHandlingUtils::application_error("Found Error", Some(e))),
//         }
//     } 
// }
