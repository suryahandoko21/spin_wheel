use async_trait::async_trait;
// use crate::domain::sp::SpinPrizesEntity;
use crate::{domain::{ spin_lists_entity::{ SpinListsPrizesEntity}, spin_prizes_entity::SpinPrizesEntity, spin_company_entity::SpinCompanyEntity}, adapters::api::{shared::response::GenericResponse, spin_lists::spin_list_payloads::{SpinListPayload, SpinPostPayload}}};
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait SpinCompanyEntityAbstract {
    async fn get_spin_company_by_id(&self,company_id: i32) -> Result<SpinCompanyEntity, Box<dyn Error>>;
    async fn get_spin_company_by_uuid(&self,company_code: String) -> Result<SpinCompanyEntity, Box<dyn Error>>;
    async fn get_spin_company_by_code(&self,company_code: String) -> Result<SpinCompanyEntity, Box<dyn Error>>;
    
}
