use async_trait::async_trait;
// use crate::domain::sp::SpinPrizesEntity;
use crate::{domain::{ spin_lists_entity::{ SpinListsPrizesEntity}, spin_promos_entity::SpinPromosEntity, spin_tickets_entity::SpinTicketsEntity}, adapters::api::{shared::response::{GenericResponse, TicketResponse, SpinAvailableResponse}, spin_lists::spin_list_payloads::SpinListPayload, spin_tickets::spin_tickets_payloads::{SpinTicketPayload, SpinTickets}}};
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait SpinTicketEntityAbstract {
    async fn post_one_spin_tickets(&self, post: &SpinTicketPayload) ->  Result<TicketResponse, Box<dyn Error>>;
    async fn get_spin_ticket_by_uuid(&self, uuid: String) ->  Result<SpinAvailableResponse, Box<dyn Error>>;
    async fn get_list_spin_ticket_by_uuid(&self, uuid: String) ->  Result<Vec<SpinTicketsEntity>, Box<dyn Error>>;
    async fn get_single_spin_ticket_by_uuid(&self, uuid: String) ->  Result<SpinTicketsEntity, Box<dyn Error>>;
    async fn used_single_spin_ticket_by_uuid(&self, uuid: String);
}