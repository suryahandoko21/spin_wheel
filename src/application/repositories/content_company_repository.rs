use async_trait::async_trait;
// use crate::domain::sp::SpinPrizesEntity;
use crate::{
    adapters::api::{content::content_payload::ContentPayload, shared::response::GenericResponse},
    domain::content_entity::ContentEntity,
};
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait ContentCompanyEntityAbstract {
    async fn get_content_by_company_by_id(
        &self,
        company_code: String,
    ) -> Result<ContentEntity, Box<dyn Error>>;
    async fn get_content_default(&self) -> Result<ContentEntity, Box<dyn Error>>;
    async fn post_contents(
        &self,
        company_code: String,
        email: String,
        post: &ContentPayload,
    ) -> Result<GenericResponse, Box<dyn Error>>;
}
