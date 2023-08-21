use actix_web::Result;
use async_trait::async_trait;
use std::error::Error;
use std::fs::File;
use crate::adapters::api::shared::enum_response::Status;
use crate::adapters::api::shared::init_global::get_hash_map_ref;
use crate::adapters::api::shared::response::GenericResponse;
use crate::adapters::api::spin_useds::spin_tickets_payloads::SpinUsedPayload;
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::api::shared::enum_response::Option;
use crate::application::repositories::spin_prizes_repository_abstract::SpinPrizesEntityAbstract;
use crate::application::repositories::spin_ticket_repository_abstract::SpinTicketEntityAbstract;
use crate::application::repositories::spin_useds_repository_abstract::SpinUsedEntityAbstract;
use rand::seq::SliceRandom; 

#[async_trait(?Send)]
impl SpinUsedEntityAbstract for ConnectionRepository {
    async fn post_one_spin_useds(&self, post: &SpinUsedPayload) ->  Result<GenericResponse, Box<dyn Error> >{
        let post_payload = post.clone();   
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
       
        //  GET ONE SPIN TICKET FOR USER WHERE SPIN SI AVAILABLE (NOT EXPIRED)
        let _spin_available_uuid = SpinTicketEntityAbstract::get_single_spin_ticket_by_uuid(self, post_payload.user_uuid).await;  
        
        //GET LIST SPIN FOR COMPANY SELECTED
        let get_all_spin_prizes_by_company_uuid = SpinPrizesEntityAbstract::get_all_spin_prizes_by_company_uuid(self,post_payload.company_uuid).await;
        
        //LIST DEFINED FOR ARRAY TO DETERMINE RANDOM CHOOSED FOR SPIN
        let mut list = Vec::new();
        for(_index,item) in get_all_spin_prizes_by_company_uuid.unwrap().iter().enumerate(){
            for _n in 0..item.percentage{
                list.push(item.prize_id);
            }
        }
        //SELECTED RANDOM CHOICE LIST  
        let spin_choosed: Vec<_> = list
        .choose_multiple(&mut rand::thread_rng(), 1)
        .collect();			

    /*
    TRY POST TO BE FOR UPDATE SPIN TICKET (IF ERROR THEN WILL PENDING AND RETRY USING CRON JOB)
    */    
    let c =  get_hash_map_ref();
    let webhook_be = &c["webhook_be"];

    let mut status = "success".to_string();
    let post_be = reqwest::get(webhook_be).await;
        if let Err(e) = post_be {
            println!("masuk pak ero{:?}",e);
            status = "pending".to_string();
        }
   let c = spin_choosed.get(0).unwrap();
   println!("c{:?}",c);
        
   let fetch_prizes_choosed = SpinPrizesEntityAbstract::get_one_spin_prize_by_id(self,**c).await;
       println!("----{:?}",fetch_prizes_choosed.ok().unwrap().prize_name); 
        println!("status = {:?}", status);
        println!("data choosed{:?}",list);       
        println!("data choosed{:?}",spin_choosed);
      
        let f = File::open("/");
        match f {
            Ok(_) => Ok(GenericResponse { status: Status::Success.to_string(),message:Option::Add.to_string()}),
        Err(e) => Err(Box::new(e)),  
        }
    }

}