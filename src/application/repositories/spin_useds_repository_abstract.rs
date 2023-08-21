use async_trait::async_trait;
// use crate::domain::sp::SpinPrizesEntity;
use crate::{domain::{ spin_lists_entity::{ SpinListsPrizesEntity}, spin_promos_entity::SpinPromosEntity, spin_tickets_entity::SpinTicketsEntity}, adapters::api::{shared::response::{GenericResponse, TicketResponse, SpinAvailableResponse}, spin_lists::spin_list_payloads::SpinListPayload, spin_tickets::spin_tickets_payloads::SpinTicketPayload, spin_useds::spin_tickets_payloads::SpinUsedPayload}};
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait SpinUsedEntityAbstract {
    async fn post_one_spin_useds(&self, post: &SpinUsedPayload) ->  Result<GenericResponse, Box<dyn Error>>;
   }