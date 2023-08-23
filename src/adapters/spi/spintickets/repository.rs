use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::mem;
use std::time::SystemTime;
use diesel::dsl::count;
use diesel::sql_types::{Integer, Text};
use diesel::{prelude::*};
use async_trait::async_trait;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use diesel::sql_query;
use crate::adapters::api::shared::enum_response::Option;
use crate::adapters::api::shared::enum_response::Status;
use crate::adapters::api::shared::response::{TicketResponse, SpinAvailableResponse};
use crate::adapters::api::spin_lists::spin_list_payloads::SpinListPayload;
use crate::adapters::api::spin_tickets::spin_tickets_payloads::SpinTicketPayload;
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::spi::cfg::schema::{tb_spin_tickets, tb_spin_prizes};
use crate::adapters::spi::cfg::{schema::tb_spin_tickets::dsl::*};

use crate::adapters::spi::prizes::models::SpinPrizes;
use crate::adapters::spi::spinlist::models::SpinListsPrizes;
use crate::adapters::spi::spintickets::models::SpinTicketsToDb;
use crate::application::mappers::db_mapper::DBMapper;
use crate::application::repositories::spin_ticket_repository_abstract::SpinTicketEntityAbstract;
use crate::domain::spin_tickets_entity::SpinTicketsEntity;
use std::sync::Arc;

use super::mappers::SpinTicketDBMapper;
use super::models::SpinTickets;



#[async_trait(?Send)]
impl SpinTicketEntityAbstract for ConnectionRepository {
    async fn post_one_spin_tickets(&self, post: &SpinTicketPayload) ->  Result<TicketResponse, Box<dyn Error>>{
        let  data =  post.clone();
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let mut message = HashMap::new();
        let mut data_one = vec![];
        for spin in data.spinTickets{
            let uuid = Arc::new(spin.uuid);
            let  prepare_data = SpinTicketsToDb{
                    user_uuid :data.userUUID.to_string() ,
                    ruleid : data.ruleid.to_string(),
                    userid : data.userid.to_string(),
                    username : data.username.to_string(),
                    ticket_id :spin.id,
                    ticket_uuid : uuid.to_string(),
                    status : spin.status,
                    pointrule_id : spin.pointRuleId,
                    expired_date : "not expired".to_string()          
            };      

            let to_vector = vec![prepare_data];
            let insert =   diesel::insert_into(tb_spin_tickets).values(&to_vector).execute(&mut conn);
            let res =  match insert {
                Ok(i) => Ok(i),
                Err(e) => Err(e),
            };
            let res = res.as_ref();
            if res.err().is_some(){
                message.insert("ticketUUID".to_string(), uuid.to_string());
                message.insert("status".to_string(), "UNPROCESSED".to_string());
                message.insert("description".to_string(), res.err().unwrap().to_string());
               
            }
            else{
                message.insert("ticketUUID".to_string(), uuid.to_string());
                message.insert("status".to_string(), "PROCESSED".to_string());
                message.insert("description".to_string(), "SUCCESS".to_string());
            }

           data_one.push(message.to_owned());
   
        }
         //for a while testing only
        let f = File::open("/");
        match f {
            Ok(_) => Ok(TicketResponse { status: Status::Success.to_string(),message:Option::Add.to_string(),data:data_one}),
            Err(e) => Err(Box::new(e)),   
        }

    }

    async fn get_spin_ticket_by_uuid(&self, uuid: String) ->  Result<SpinAvailableResponse, Box<dyn Error>>{
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let uuid_clone = uuid.clone();
        let result = tb_spin_tickets.filter(user_uuid.eq(&uuid_clone)).get_result::<SpinTickets>(&mut conn);
        let query = tb_spin_tickets.filter(user_uuid.eq(&uuid_clone)).select(count(id)).get_result::<i64>(&mut conn);
        match result {
            Ok(_)=>Ok(SpinAvailableResponse{message:"ewew".to_string(),status:"SUxxess".to_string(),spin:query.unwrap()}),
            Err(_) => Ok(SpinAvailableResponse{message:"ewew".to_string(),status:"SUxxess".to_string(),spin:query.unwrap()}),
        }

    }

    async fn get_list_spin_ticket_by_uuid(&self, uuid: String) ->  Result<Vec<SpinTicketsEntity>, Box<dyn Error>>{
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let uuid_clone = uuid.clone();
        let results = tb_spin_tickets.filter(user_uuid.eq(&uuid_clone)).load::<SpinTickets>(&mut conn);
        match results {
            Ok(models) => Ok(models.into_iter().map(SpinTicketDBMapper::to_entity).collect::<Vec<SpinTicketsEntity>>()),
            Err(e) => Err(Box::new(e)),
        }

    }
    async fn get_single_spin_ticket_by_uuid(&self, uuid: String) ->  Result<SpinTicketsEntity, Box<dyn Error>>{
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let list_spins = sql_query("select * from tb_spin_tickets where status='AVAILABLE' and user_uuid=$1 ORDER BY ID ASC LIMIT 1").bind::<Text,_>(uuid).get_result::<SpinTickets>(&mut conn);
        match  list_spins
        {
           Ok(models) => Ok(SpinTicketDBMapper::to_entity(models)),
           Err(e) => Err(Box::new(e)),
       }
    }
   
    async fn used_single_spin_ticket_by_uuid(&self, uuid: String){
        println!("masuk pak eoko{:?}",uuid);
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let _update = diesel::update(tb_spin_tickets.filter(ticket_uuid.eq(uuid))).set(status.eq("TICKET USED")).execute(&mut conn);

    }
}