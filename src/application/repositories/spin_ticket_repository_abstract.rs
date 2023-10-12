use async_trait::async_trait;
// use crate::domain::sp::SpinPrizesEntity;
use crate::{domain::spin_tickets_entity::SpinTicketsEntity, adapters::api::{shared::response::{TicketResponse, SpinAvailableResponse}, spin_tickets::spin_tickets_payloads::SpinTicketPayload}};
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait SpinTicketEntityAbstract {
    async fn post_one_spin_tickets(&self, companies_code:String,post: &SpinTicketPayload) ->  Result<TicketResponse, Box<dyn Error>>;
    async fn get_spin_ticket_by_uuid(&self, uuid: String) ->  Result<SpinAvailableResponse, Box<dyn Error>>;
    async fn get_spin_ticket_by_userid(&self, userid: String) ->  Result<SpinAvailableResponse, Box<dyn Error>>;
    async fn get_list_spin_ticket_by_uuid(&self, uuid: String) ->  Result<Vec<SpinTicketsEntity>, Box<dyn Error>>;
    async fn get_single_spin_ticket_by_uuid(&self, uuid: String,company_code:String) ->  Result<SpinTicketsEntity, Box<dyn Error>>;
    async fn used_single_spin_ticket_by_uuid(&self, uuid: String);
    async fn get_spin_ticket_by_uuid_company_code(&self, uuid: String,company_code:String) ->  Result<SpinAvailableResponse, Box<dyn Error>>;
   
}