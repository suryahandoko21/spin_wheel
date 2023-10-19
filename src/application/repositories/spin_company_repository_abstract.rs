use async_trait::async_trait;
// use crate::domain::sp::SpinPrizesEntity;
use crate::domain::spin_company_entity::SpinCompanyEntity;
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait SpinCompanyEntityAbstract {
    async fn get_spin_company_by_id(
        &self,
        company_id: i32,
    ) -> Result<SpinCompanyEntity, Box<dyn Error>>;
    async fn get_spin_company_by_uuid(
        &self,
        company_code: String,
    ) -> Result<SpinCompanyEntity, Box<dyn Error>>;
    async fn get_spin_company_by_code(
        &self,
        company_code: String,
    ) -> Result<SpinCompanyEntity, Box<dyn Error>>;
    async fn fetch_spin_company_from_url(&self) -> bool;
}
