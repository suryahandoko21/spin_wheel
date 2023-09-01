
use std::error::Error;


use async_trait::async_trait;

use crate::adapters::api::shared::response::GenericResponse;
use crate::application::repositories::cron_repository_abstract::CronEntityAbstract;

use crate::adapters::spi::cfg::db_connection::ConnectionRepository;







// use crate::application::repositories::cron_repository_abstract::CronEntityAbstract;
#[async_trait(?Send)]
impl CronEntityAbstract for ConnectionRepository {
    async fn check_pending_post_be(&self)->Result<GenericResponse, Box<dyn Error>>{
        println!("dasdasda");

        Ok(GenericResponse { status: "ok".to_string(), message: "ok".to_string() })
        // println!("dasda");
        // let result:Result<Vec<FailedProcess>,_> = tb_spin_failed_process.filter(status.eq("used")).load::<FailedProcess>(&mut CONN.get().unwrap().get().expect("cant connect database"));
        //     for i in result.iter(){
        //         for data in i.iter(){
        //             println!("sssss");
        //             let request_be = RequestBeResult{
        //                 ticketUuid : data.ticket_uuid.to_string(),
        //                 userId : data.user_id.to_string(),
        //                 rewardName : data.reward_name.to_string(),
        //                 status : "used".to_string(),
        //                 rewardType: data.reward_type.to_string(),
        //                 money : data.money
        //             };
        //             let post_request = re_post_to_be(request_be);
        //             // println!("podas{:?}",post_request);
        //             // if post_request.await {
        //             //     println!("success send to be");
        //             // }
        //             // else{
        //             //     println!("failed bro ");
        //             // }
        //         }
        //     //    }
        //     }
        
    }
}

