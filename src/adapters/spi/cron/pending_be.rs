use diesel::prelude::*;
use crate::adapters::api::shared::request_be::RequestBeResult;
use crate::adapters::spi::cfg::pg_connection::CONN;
use crate::adapters::spi::cfg::schema::tb_spin_failed_process::dsl::*;
use crate::adapters::spi::failed::models::FailedProcess;
use crate::adapters::spi::used::post_be::post_to_be;

pub async fn process_for_pending_be(){
 let result:Result<Vec<FailedProcess>,_> = tb_spin_failed_process.filter(status.eq("used")).load::<FailedProcess>(&mut CONN.get().unwrap().get().expect("cant connect database"));
            for i in result.iter(){
                for data in i.iter(){
                    let request_be = RequestBeResult{
                        ticketUuid : data.ticket_uuid.to_string(),
                        userId : data.user_id.to_string(),
                        rewardName : data.reward_name.to_string(),
                        status : "used".to_string(),
                        rewardType: data.reward_type.to_string(),
                        money : data.money
                    };
                    let post_request = post_to_be(request_be).await;
                    if post_request {
                        
                    }
        
                }
            }
}
