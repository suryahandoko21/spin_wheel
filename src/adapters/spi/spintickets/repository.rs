use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::mem;
use std::time::SystemTime;

use async_trait::async_trait;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use diesel::sql_query;
use crate::adapters::api::shared::enum_response::Option;
use crate::adapters::api::shared::enum_response::Status;
use crate::adapters::api::shared::response::TicketResponse;
use crate::adapters::api::spin_lists::spin_list_payloads::SpinListPayload;
use crate::adapters::api::spin_tickets::spin_tickets_payloads::SpinTicketPayload;
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::spi::cfg::schema::tb_spin_tickets;
use crate::adapters::spi::cfg::{schema::tb_spin_tickets::dsl::*};

use crate::adapters::spi::spintickets::models::SpinTicketsToDb;
use crate::application::repositories::spin_ticket_repository_abstract::SpinTicketEntityAbstract;




#[async_trait(?Send)]
impl SpinTicketEntityAbstract for ConnectionRepository {
    async fn post_one_spin_tickets(&self, post: &SpinTicketPayload) ->  Result<TicketResponse, Box<dyn Error>>{
        let  data =  post.clone();
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let mut message = HashMap::new();
        // let mut scores = HashMap::new();
        // let mut xs:TicketResponse = vec![];
        let mut data_one = vec![];
        for x in data.spinTickets{
            let  prepare_data = SpinTicketsToDb{
                user_uuid :data.userUUID.to_string() ,
                ruleid : data.ruleid.to_string(),
                userid : data.userid.to_string(),
                username : data.username.to_string(),
                ticket_id :x.id,
                ticket_uuid : x.uuid,
                status : x.status,
                pointrule_id : x.pointRuleId,
                expired_date : "expired".to_string()          
            };       
            let to_vector = vec![prepare_data];
            let insert =   diesel::insert_into(tb_spin_tickets).values(&to_vector).execute(&mut conn);
            let  res =  match insert {
                Ok(i) => Ok(i),
                Err(e) => Err(e),
            };
            let res = res.as_ref();
            if res.err().is_some(){
                message.insert("ticketUUID".to_string(), x.id.to_string());
                message.insert("status".to_string(), "UNPROCESSED".to_string());
                message.insert("description".to_string(), res.err().unwrap().to_string());
               
            }
            else{
                message.insert("ticketUUID".to_string(), x.id.to_string());
                message.insert("status".to_string(), "PROCESSED".to_string());
                message.insert("description".to_string(), "SUCCESS".to_string());
            }
           data_one.push(message.to_owned());
   
        }
        let f = File::open("/");

        match f {
            Ok(_) => Ok(TicketResponse { status: Status::Success.to_string(),message:Option::Add.to_string(),data:data_one}),
        Err(e) => Err(Box::new(e)),   
        }

    }


}