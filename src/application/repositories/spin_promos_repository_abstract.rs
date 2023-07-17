use async_trait::async_trait;
use crate::{domain::{spin_promos_entity::SpinPromosEntity}, adapters::api::{spin_promos::spin_promos_payloads::SpinPromosPayload, shared::response::GenericResponse}};
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;
#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait SpinPromosEntityAbstract {
    async fn get_all_spin_promos(&self) -> Result<Vec<SpinPromosEntity>, Box<dyn Error>>;
    async fn post_one_spin_promos(&self, post: &SpinPromosEntity) ->  Result<GenericResponse, Box<dyn Error>>;
  
}
