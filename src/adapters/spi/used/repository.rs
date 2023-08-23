use actix_web::Result;
use async_trait::async_trait;
use std::error::Error;
use std::time::SystemTime;
use crate::adapters::api::shared::enum_response::Status;
use crate::adapters::api::shared::init_global::GLOBAL_MAP;
use crate::adapters::api::shared::request_be::RequestBeResult;
use crate::adapters::api::shared::response::GenericResponse;
use crate::adapters::api::spin_useds::spin_tickets_payloads::SpinUsedPayload;
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::spi::used::models::SpinUsedsToDb;
use crate::adapters::spi::used::post_be::post_to_be;
use crate::application::repositories::spin_company_repository_abstract::SpinCompanyEntityAbstract;
use crate::application::repositories::spin_prizes_repository_abstract::SpinPrizesEntityAbstract;
use crate::application::repositories::spin_ticket_repository_abstract::SpinTicketEntityAbstract;
use crate::application::repositories::spin_useds_repository_abstract::SpinUsedEntityAbstract;
use rand::seq::SliceRandom; 
use crate::adapters::spi::cfg::schema::tb_spin_used::dsl::*;
use diesel::RunQueryDsl;
use std::sync::Arc;
#[async_trait(?Send)]
impl SpinUsedEntityAbstract for ConnectionRepository {
    async fn post_one_spin_useds(&self, post: &SpinUsedPayload) ->  Result<GenericResponse, Box<dyn Error> >{
        let post_payload =Arc::new(post);   
        let uuid =  post_payload.user_uuid.to_string();
        let company_uuid = post_payload.company_uuid.to_string();
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let get_company = SpinCompanyEntityAbstract::get_spin_company_by_uuid(self, company_uuid.to_string()).await;
    
        /*  GET ONE SPIN TICKET FOR USER WHERE SPIN SI AVAILABLE (NOT EXPIRED) */
        let spin_available_uuid = SpinTicketEntityAbstract::get_single_spin_ticket_by_uuid(self, uuid.to_string()).await;  
        
        /* GET LIST SPIN FOR COMPANY SELECTED */
        let get_all_spin_prizes_by_company_uuid = SpinPrizesEntityAbstract::get_all_spin_prizes_by_company_uuid(self,company_uuid.to_string()).await;
        
        /*LIST DEFINED FOR ARRAY TO DETERMINE RANDOM CHOOSED FOR SPIN*/
        let mut list = Vec::new();
        for(_index,item) in get_all_spin_prizes_by_company_uuid.unwrap().iter().enumerate(){
            for _n in 0..item.percentage{
                if item.prize_amount > 0{
                    list.push(item.prize_id);
                }     
            }
        }
      
        /* SELECTED RANDOM CHOICE LIST */  
        let spin_choosed: Vec<_> = list
        .choose_multiple(&mut rand::thread_rng(), 1)
        .collect();			

        /*
        TRY POST TO BE FOR UPDATE SPIN TICKET (IF ERROR THEN WILL PENDING AND RETRY USING CRON JOB)
        */    
        let global_map =GLOBAL_MAP.get().unwrap();
        let webhook_be = &global_map["webhook_be"];

        let mut status = "failed".to_string();
        let  choosed = spin_choosed.get(0).unwrap();
            
        let prizes_choosed = SpinPrizesEntityAbstract::get_one_spin_prize_by_id(self,**choosed).await;
        let data_prizes =  Arc::new(prizes_choosed.ok().unwrap());
        let prizes_id = data_prizes.prize_id;
        let prize_name = &data_prizes.prize_name;
        let ticket_uuid = Arc::new(spin_available_uuid.ok().unwrap().ticket_uuid);
        let request_be = RequestBeResult{
            prize: prize_name.to_string(),
            ticket_uuid: ticket_uuid.to_string()
        };

        /* update ticked to status */
        // let _= SpinTicketEntityAbstract::used_single_spin_ticket_by_uuid(self,ticket_uuid.to_string()).await;
        /* reduce amount prize used */
        let _= SpinPrizesEntityAbstract::used_one_spin_by_prize_id(self, prizes_id).await;
       
        /* POST REQUEST TO BE */
        let post_request = post_to_be(webhook_be.to_string(),request_be);
        if post_request.await {
            status = "success".to_string();
        }

        let prepare_data = SpinUsedsToDb{
             user_id : uuid.to_string(), 
             created_at : SystemTime::now(), 
             updated_at : SystemTime::now(),
             created_by : "System".to_string(),
             updated_by : "System".to_string(),
             used_status : status,
             prize_id : **choosed,
             company_id :get_company.ok().unwrap().id
        };

        let to_vector = vec![prepare_data];   
        let insert =   diesel::insert_into(tb_spin_used).values(&to_vector).execute(&mut conn);
        match insert {
                 Ok(_) => Ok(GenericResponse { status: Status::Success.to_string(),message: prize_name.to_string()}),
                 Err(e) => Err(Box::new(e)),  
            }
        }

}