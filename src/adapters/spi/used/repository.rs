use actix_web::Result;
use async_trait::async_trait;
use std::error::Error;
use std::time::SystemTime;
use crate::adapters::api::shared::request_be::RequestBeResult;
use crate::adapters::api::shared::response::SpinResponse;
use crate::adapters::api::shared::selected_enum::select_enum_reward;
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
use std::sync::Arc;
#[async_trait(?Send)]
impl SpinUsedEntityAbstract for ConnectionRepository {
    async fn post_one_spin_useds(&self, post: &SpinUsedPayload) ->  Result<SpinResponse, Box<dyn Error> >{
        let post_payload =Arc::new(post);   
        let uuid =  Arc::new(post_payload.user_uuid.to_string());
        let company_code = post_payload.company_code.to_string();
               
        /*  GET ONE SPIN TICKET FOR USER WHERE SPIN SI AVAILABLE (NOT EXPIRED) */
        let spin_available_uuid = SpinTicketEntityAbstract::get_single_spin_ticket_by_uuid(self, uuid.to_string()).await;  
        
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
        /*
        TRY POST TO BE FOR UPDATE SPIN TICKET (IF ERROR THEN WILL PENDING AND RETRY USING CRON JOB)
        */    
        // let mut response_message = "".to_string();
        let mut status = "failed".to_string();
        let mut reward_name = "".to_string();
        let mut reward_type = "".to_string();
        let mut reward_description = "".to_string();
        let mut status_spin = "Anda tidak memiliki koin".to_string();
        if spin_choosed.get(0).is_some(){
            // response_message = "None".to_string();
            let choosed = spin_choosed.get(0).unwrap();
            let reward_choosed = SpinRewardEntityAbstract::get_one_spin_reward_by_id(self,**choosed).await;
            let data_reward =  reward_choosed.ok().unwrap();
            let reward_id = data_reward.reward_id;  
            let spin_avail = spin_available_uuid.as_ref();
            if spin_avail.ok().is_some(){
                status_spin = "OK".to_string();
                reward_name = String::from(&data_reward.reward_name);
                reward_description = String::from(&data_reward.reward_note);
                reward_type = String::from(&data_reward.reward_category); 
                let ticket_id = &spin_avail.ok().unwrap().ticket_uuid;
                let _= SpinTicketEntityAbstract::used_single_spin_ticket_by_uuid(self,ticket_id.to_string()).await; 
                /* reduce amount award used */
                let _= SpinRewardEntityAbstract::used_one_spin_by_reward_id(self, reward_id).await;
                let request_be = RequestBeResult{
                        ticketUuid : ticket_id.to_string(),
                        userId : uuid.to_string(),
                        rewardName : reward_name.to_string(),
                        status : "used".to_string(),
                        rewardDescriptions:reward_description.to_string(),
                        rewardType: select_enum_reward(reward_type.to_string()),
                        money : data_reward.reward_money
                    };
                /* POST REQUEST TO BE */
                let post_request = post_to_be(request_be);
                if post_request.await {
                    let success_post = ProcessSuccessToDb{
                        ticket_uuid : ticket_id.to_string(),
                        user_id : uuid.to_string(),
                        reward_name : reward_name.to_string(),
                        status : "used".to_string(),
                        reward_type: reward_type.to_string(),
                        money : data_reward.reward_money,
                        post_status : "success".to_string(),
                        created_at : SystemTime::now()
                        };       
                    SuccessProcessEntityAbstract::post_success_proccess(self,success_post).await;
                    status = "success".to_string();
                }
                else{
                    let failed_post = FailedProcessToDb{
                            ticket_uuid : ticket_id.to_string(),
                            user_id : uuid.to_string(),
                            reward_name : reward_name.to_string(),
                            reward_description :reward_description.to_string(),    
                            status :"used".to_string(),
                            reward_type: reward_type.to_string(),
                            money : data_reward.reward_money,
                            post_status : "failed".to_string(),
                            created_at : SystemTime::now(),
                            updated_at:SystemTime::now()
                        };   
                    FailedProcessEntityAbstract::post_failed_proccess(self,failed_post).await; 
                }
                let prepare_data = SpinUsedsToDb{
                        user_id : uuid.to_string(), 
                        created_at : SystemTime::now(), 
                        updated_at : SystemTime::now(),
                        created_by : "System".to_string(),
                        updated_by : "System".to_string(),
                        used_status : status,
                        prize_id : **choosed,
                        companies_code : company_code,
                        ticket_uuid: ticket_id.to_string()
                    };
            let to_vector = vec![prepare_data];   
            let _ = diesel::insert_into(tb_spin_used).values(&to_vector).execute(&mut CONN.get().unwrap().get().expect("Failed connect database"));
             }
        }
        // Ok(Spi { status: Status::Success.to_string(),message:reward_name.to_string()})
        Ok(SpinResponse { status: status_spin.to_string(), category: reward_type.to_string(), reward: reward_name, description: reward_description })
}
}