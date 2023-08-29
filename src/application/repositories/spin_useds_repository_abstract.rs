use async_trait::async_trait;
// use crate::domain::sp::SpinPrizesEntity;
use crate::adapters::api::{shared::response::GenericResponse, spin_useds::spin_tickets_payloads::SpinUsedPayload};
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait SpinUsedEntityAbstract {
    async fn post_one_spin_useds(&self, post: &SpinUsedPayload) ->  Result<GenericResponse, Box<dyn Error>>;
   }