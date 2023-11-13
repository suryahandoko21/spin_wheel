use crate::{
    adapters::api::{content::content_payload::ContentPayload, shared::response::GenericResponse},
    application::{
        repositories::content_company_repository::ContentCompanyEntityAbstract,
        usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils,
    },
    domain::error::ApiError,
};
use async_trait::async_trait;

pub struct PostContentByCompannyCodeUseCase<'a> {
    company_code: &'a String,
    email: &'a String,
    remote_ip: &'a String,
    post: &'a ContentPayload,
    repository: &'a dyn ContentCompanyEntityAbstract,
}

impl<'a> PostContentByCompannyCodeUseCase<'a> {
    pub fn new(
        company_code: &'a String,
        email: &'a String,
        remote_ip: &'a String,
        post: &'a ContentPayload,
        repository: &'a dyn ContentCompanyEntityAbstract,
    ) -> Self {
        PostContentByCompannyCodeUseCase {
            repository,
            company_code,
            email,
            remote_ip,
            post,
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<GenericResponse> for PostContentByCompannyCodeUseCase<'a> {
    async fn execute(&self) -> Result<GenericResponse, ApiError> {
        let post_content = self
            .repository
            .post_contents(
                self.company_code.to_string(),
                self.email.to_string(),
                self.remote_ip.to_string(),
                self.post,
            )
            .await;
        match post_content {
            Ok(facts) => Ok(facts),
            Err(e) => Err(ErrorHandlingUtils::application_error(
                "Cannot get all DATA",
                Some(e),
            )),
        }
    }
}
