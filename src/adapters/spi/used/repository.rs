use actix_web::Result;
use async_trait::async_trait;
use std::error::Error;
use std::time::SystemTime;
use crate::adapters::api::shared::request_be::RequestBeResult;
use crate::adapters::api::shared::response::SpinResponse;
use crate::adapters::api::spin_useds::spin_tickets_payloads::SpinUsedPayload;
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::spi::cfg::pg_connection::CONN;
use crate::adapters::spi::failed::models::FailedProcessToDb;
use crate::adapters::spi::success::models::ProcessSuccessToDb;
use crate::adapters::spi::used::models::SpinUsedsToDb;
use crate::adapters::spi::used::post_be::post_to_be;
use crate::application::repositories::failed_proccess_abstract::FailedProcessEntityAbstract;
use crate::application::repositories::spin_rewards_repository_abstract::SpinRewardEntityAbstract;
use crate::application::repositories::spin_ticket_repository_abstract::SpinTicketEntityAbstract;
use crate::application::repositories::spin_useds_repository_abstract::SpinUsedEntityAbstract;
use crate::application::repositories::success_process_abstract::SuccessProcessEntityAbstract;
use rand::seq::SliceRandom; 
use crate::adapters::spi::cfg::schema::tb_spin_used::dsl::*;
use diesel::RunQueryDsl;
#[async_trait(?Send)]
impl SpinUsedEntityAbstract for ConnectionRepository {
    async fn post_one_spin_useds(&self, post: &SpinUsedPayload,company_code:String,url_addresses:String) ->  Result<SpinResponse, Box<dyn Error> >{
        let post_payload =&post;   
        let uuid = post_payload.user_uuid.to_string();
        let url_addresses = url_addresses.to_string();      
        /*  GET ONE SPIN TICKET FOR USER WHERE SPIN SI AVAILABLE (NOT EXPIRED) */
        let spin_available_uuid = SpinTicketEntityAbstract::get_single_spin_ticket_by_uuid(self, uuid.to_string(),company_code.to_string()).await;  
        /* GET LIST SPIN FOR COMPANY SELECTED */
        let get_all_spin_reward_by_company_code = SpinRewardEntityAbstract::get_all_spin_reward_by_company_code_by_status(self,company_code.to_string()).await;
        /*LIST DEFINED FOR ARRAY TO DETERMINE RANDOM CHOOSED FOR SPIN*/
        let mut list = Vec::new();
        for(_index,item) in get_all_spin_reward_by_company_code.unwrap().iter().enumerate(){
            for _n in 0..item.percentage{
                if item.reward_amount > 0{
                    list.push(item.reward_id);
                }     
            }
        }
        /* SELECTED RANDOM CHOICE LIST */  
        let spin_choosed: Vec<_> = list
        .choose_multiple(&mut rand::thread_rng(), 1)
        .collect();			
        let mut response = SpinResponse{
            status :"".to_string(),
            reward:None
        };

        let spin_avail = spin_available_uuid.as_ref();
        let choosed ;
        if !spin_choosed.get(0).is_some() || !spin_avail.ok().is_some(){
            let obj_zonk = SpinRewardEntityAbstract::get_one_zonk_spin_reward_by_company(self,company_code.to_string()).await;
            let result_zonk = obj_zonk.ok().unwrap();
            choosed = result_zonk.reward_id;
            response.reward= Some(result_zonk);
            response.status = "NONE".to_string();
        }
        else{
            let reward_id = spin_choosed.get(0).unwrap();
            let reward_choosed = SpinRewardEntityAbstract::get_one_spin_reward_by_id(self,**reward_id).await;
            let data_reward =  reward_choosed.ok().unwrap();
            choosed = **reward_id;
            response.reward=Some(data_reward);
            response.status = "success".to_string();
        }
        
        let reward = response.reward.as_ref().unwrap();
        let reward_id = &reward.reward_id;  
        let reward_name = String::from(&reward.reward_name);
        let reward_description = String::from(&reward.reward_note);
        let reward_type = String::from(&reward.reward_category); 
        let ticket_id = &spin_avail.ok().unwrap().ticket_uuid;

        let request_be = RequestBeResult{
            ticketUuid : ticket_id.to_string(),
            userUuid : uuid.to_string(),
            rewardName : reward_name.to_string(),
            status : "used".to_string(),
            rewardDescriptions:reward_description.to_string(),
            rewardType: reward_type.to_string(),
            money : reward.reward_money
        };
        let _= SpinTicketEntityAbstract::used_single_spin_ticket_by_uuid(self,ticket_id.to_string()).await;      
        /* TRY POST TO BE FOR UPDATE SPIN TICKET (IF ERROR THEN WILL PENDING AND RETRY USING CRON JOB) */    
        let (status_post, status, message,status_code) = post_to_be(request_be,url_addresses.to_string()).await;
        if status_post {
            let success_post = ProcessSuccessToDb{
                ticket_uuid : ticket_id.to_string(),
                user_id : uuid.to_string(),
                reward_name : reward_name.to_string(),
                status : "used".to_string(),
                reward_type: reward_type.to_string(),
                money : reward.reward_money,
                post_status : "success".to_string(),
                created_at : SystemTime::now()
                };       
            SuccessProcessEntityAbstract::post_success_proccess(self,success_post).await;
            let prepare_data = SpinUsedsToDb{
                user_id : uuid.to_string(), 
                created_at : SystemTime::now(), 
                updated_at : SystemTime::now(),
                created_by : "System".to_string(),
                updated_by : "System".to_string(),
                used_status : status,
                prize_id : choosed,
                companies_code : company_code,
                ticket_uuid: ticket_id.to_string()
                };
            let to_vector = vec![prepare_data]; 
            let _ = diesel::insert_into(tb_spin_used).values(&to_vector).execute(&mut CONN.get().unwrap().get().expect("Failed connect database"));    
            /* reduce amount award used */
            let _= SpinRewardEntityAbstract::used_one_spin_by_reward_id(self, *reward_id).await;  
        }else{
            let obj_zonk = SpinRewardEntityAbstract::get_one_zonk_spin_reward_by_company(self,company_code.to_string()).await;
            let result_zonk = obj_zonk.ok().unwrap();
            let mut failed_post_status = "rejected".to_string();
            if status_code == 504{
                failed_post_status = "failed".to_string();
            }
            let failed_post = FailedProcessToDb{
                ticket_uuid : ticket_id.to_string(),
                user_id : uuid.to_string(),
                reward_name : result_zonk.reward_name.to_string(),
                reward_description :result_zonk.reward_note.to_string(),    
                status :"used".to_string(),
                reward_type: result_zonk.reward_category.to_string(),
                money : result_zonk.reward_money,
                post_status : failed_post_status.to_string(),
                failed_message : message,
                url_address: url_addresses.to_string(),
                created_at : SystemTime::now(),
                updated_at:SystemTime::now()
                };   
            FailedProcessEntityAbstract::post_failed_proccess(self,failed_post).await; 
            let prepare_data = SpinUsedsToDb{
                user_id : uuid.to_string(), 
                created_at : SystemTime::now(), 
                updated_at : SystemTime::now(),
                created_by : "System".to_string(),
                updated_by : "System".to_string(),
                used_status : failed_post_status.to_string(),
                prize_id : result_zonk.reward_id,
                companies_code : company_code,
                ticket_uuid: ticket_id.to_string()
                };
                let to_vector = vec![prepare_data];   
                let _ = diesel::insert_into(tb_spin_used).values(&to_vector).execute(&mut CONN.get().unwrap().get().expect("Failed connect database"));      
       }     
      
        Ok(response)
}
}