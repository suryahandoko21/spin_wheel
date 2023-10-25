use async_trait::async_trait;
// use crate::domain::sp::SpinPrizesEntity;
use crate::adapters::api::{
    shared::response::SpinResponse, spin_useds::spin_tickets_payloads::SpinUsedPayload,
};
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait SpinUsedEntityAbstract {
    async fn post_one_spin_useds(
        &self,
        post: &SpinUsedPayload,
        email: String,
        companies_code: String,
        url_addresses: String,
        remote_ip: String,
    ) -> Result<SpinResponse, Box<dyn Error>>;
}
