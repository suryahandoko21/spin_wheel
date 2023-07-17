use async_trait::async_trait;
// use crate::domain::sp::SpinPrizesEntity;
use crate::{domain::{ spin_lists_entity::{ SpinListsPrizesEntity}, spin_promos_entity::SpinPromosEntity, spin_tikcets_entity::SpinTicketsEntity}, adapters::api::{shared::response::{GenericResponse, TicketResponse}, spin_lists::spin_list_payloads::SpinListPayload, spin_tickets::spin_tickets_payloads::SpinTicketPayload}};
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait SpinTicketEntityAbstract {
    async fn post_one_spin_tickets(&self, post: &SpinTicketPayload) ->  Result<TicketResponse, Box<dyn Error>>;
}