use super::mappers::SpinTicketDBMapper;
use super::models::SpinTickets;
use crate::adapters::api::shared::enum_response::Options;
use crate::adapters::api::shared::response::{SpinAvailableResponse, TicketResponse};
use crate::adapters::api::spin_tickets::spin_tickets_payloads::SpinTicketPayload;
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::spi::cfg::pg_connection::CONN;
use crate::adapters::spi::cfg::schema::tb_spin_tickets::dsl::*;
use crate::adapters::spi::spintickets::models::SpinTicketsToDb;
use crate::application::mappers::db_mapper::DBMapper;
use crate::application::repositories::spin_ticket_repository_abstract::SpinTicketEntityAbstract;
use crate::domain::spin_tickets_entity::SpinTicketsEntity;
use async_trait::async_trait;
use diesel::dsl::count;
use diesel::sql_query;
use diesel::sql_types::Text;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::collections::HashMap;
use std::error::Error;

#[async_trait(?Send)]
impl SpinTicketEntityAbstract for ConnectionRepository {
    async fn post_one_spin_tickets(
        &self,
        companies_code: String,
        post: &SpinTicketPayload,
    ) -> Result<TicketResponse, Box<dyn Error>> {
        let data = &post;
        let mut message = HashMap::new();
        let mut data_one = vec![];
        let incoming_data = data.spinTickets.len();
        let mut length_success = 0;
        for spin in &data.spinTickets {
            let uuid = spin.uuid.to_string();
            let prepare_data: SpinTicketsToDb = SpinTicketsToDb {
                user_uuid: data.userUuId.to_string(),
                userid: spin.userId.to_string(),
                username: data.username.to_string(),
                ticket_id: spin.id,
                ticket_uuid: uuid.to_string(),
                status: spin.status.to_string(),
                pointrule_id: spin.pointRuleId,
                expired_date: spin.ticketExpiredDate.as_ref().unwrap_or(&"".to_string()).to_string(),
                pointrule_name: spin.pointRuleName.to_string(),
                ticket_number: spin.ticketNumber.to_string(),
                expired_type: spin.expiredType.to_string(),
                expired_value: spin.expiredValue,
                created_date: spin.ticketCreatedDate.to_string(),
                is_payment_gateway: spin.isPaymentGateWay,
                company_code: companies_code.to_string(),
                send_be: false,
            };
            let to_vector = vec![prepare_data];
            let insert = diesel::insert_into(tb_spin_tickets)
                .values(&to_vector)
                .execute(&mut CONN.get().unwrap().get().expect("failed connect db"));
            let res = match insert {
                Ok(i) => Ok(i),
                Err(e) => Err(e),
            };
            let res = res.as_ref();

            if res.err().is_some() {
                message.insert("ticketUUID".to_string(), uuid.to_string());
                message.insert("status".to_string(), "UNPROCESSED".to_string());
                message.insert("description".to_string(), res.err().unwrap().to_string());
            } else {
                length_success += 1;
                message.insert("ticketUUID".to_string(), uuid.to_string());
                message.insert("status".to_string(), "PROCESSED".to_string());
                message.insert("description".to_string(), "SUCCESS".to_string());
            }
            data_one.push(message.to_owned());
        }
        let mut response_status = "SUCCESS_ALL";
        if length_success == 0 {
            response_status = "FAILED_ALL";
        } else if length_success < incoming_data {
            response_status = "PARTIAL";
        }
        Ok(TicketResponse {
            status: response_status.to_string(),
            message: Options::Add.to_string(),
            data: data_one,
        })
    }

    async fn get_spin_ticket_by_userid(
        &self,
        userids: String,
    ) -> Result<SpinAvailableResponse, Box<dyn Error>> {
        let limit_spin_user = true;
        let query = tb_spin_tickets
            .filter(user_uuid.eq(userids))
            .filter(status.eq("AVAILABLE"))
            .select(count(id))
            .get_result::<i64>(&mut CONN.get().unwrap().get().expect("failed connect db"));
        Ok(SpinAvailableResponse {
            message: "Spin Available".to_string(),
            spin_amount: query.unwrap(),
            available: limit_spin_user,
        })
    }
    async fn get_spin_ticket_by_uuid(
        &self,
        uuid: String,
    ) -> Result<SpinAvailableResponse, Box<dyn Error>> {
        let limit_spin_user = true;
        let query = tb_spin_tickets
            .filter(user_uuid.eq(uuid))
            .filter(status.eq("AVAILABLE"))
            .select(count(id))
            .get_result::<i64>(&mut CONN.get().unwrap().get().expect("failed connect db"));
        Ok(SpinAvailableResponse {
            message: "Spin Available".to_string(),
            spin_amount: query.unwrap(),
            available: limit_spin_user,
        })
    }

    async fn get_spin_ticket_by_uuid_company_code(
        &self,
        uuid: String,
        companies_code: String,
    ) -> Result<SpinAvailableResponse, Box<dyn Error>> {
        let limit_spin_user = true;
        let query = tb_spin_tickets
            .filter(user_uuid.eq(uuid))
            .filter(company_code.eq(companies_code))
            .filter(status.eq("AVAILABLE"))
            .select(count(id))
            .get_result::<i64>(&mut CONN.get().unwrap().get().expect("failed connect db"));
        Ok(SpinAvailableResponse {
            message: "Spin Available".to_string(),
            spin_amount: query.unwrap(),
            available: limit_spin_user,
        })
    }
    async fn get_list_spin_ticket_by_uuid(
        &self,
        uuid: String,
    ) -> Result<Vec<SpinTicketsEntity>, Box<dyn Error>> {
        let results = tb_spin_tickets
            .filter(user_uuid.eq(&uuid))
            .load::<SpinTickets>(&mut CONN.get().unwrap().get().expect("failed connect db"));
        match results {
            Ok(models) => Ok(models
                .into_iter()
                .map(SpinTicketDBMapper::to_entity)
                .collect::<Vec<SpinTicketsEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }
    async fn get_single_spin_ticket_by_uuid(
        &self,
        uuid: String,
        companies_code: String,
    ) -> Result<SpinTicketsEntity, Box<dyn Error>> {
        let list_spins = sql_query("select * from tb_spin_tickets where status='AVAILABLE' and user_uuid=$1 and company_code=$2 ORDER BY ID ASC LIMIT 1")
                        .bind::<Text,_>(uuid)
                        .bind::<Text,_>(companies_code)
                        .get_result::<SpinTickets>(&mut CONN.get().unwrap().get().expect("failed connect db"));
        match list_spins {
            Ok(models) => Ok(SpinTicketDBMapper::to_entity(models)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn used_single_spin_ticket_by_uuid(&self, uuid: String) {
        let _update = diesel::update(tb_spin_tickets.filter(ticket_uuid.eq(uuid)))
            .set(status.eq("TICKET USED"))
            .execute(&mut CONN.get().unwrap().get().expect("failed connect db"));
    }
}
