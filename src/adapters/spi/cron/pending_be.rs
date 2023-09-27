use std::sync::Arc;
use std::time::SystemTime;

use diesel::prelude::*;
use crate::adapters::api::shared::request_be::RequestBeResult;
use crate::adapters::api::shared::selected_enum::select_enum_reward;
use crate::adapters::spi::cfg::pg_connection::CONN;
use crate::adapters::spi::cfg::schema::tb_spin_failed_process::dsl::*;
use crate::adapters::spi::failed::models::FailedProcess;
use crate::adapters::spi::used::post_be::post_to_be;
use crate::adapters::spi::cfg::schema::tb_spin_used::dsl::*;
pub async fn process_for_pending_be(){
 let result:Result<Vec<FailedProcess>,_> = tb_spin_failed_process.filter(post_status.eq("failed")).load::<FailedProcess>(&mut CONN.get().unwrap().get().expect("cant connect database"));
            for i in result.iter(){
                for data in i.iter(){
                    let ticket_uuids =  Arc::new(data.ticket_uuid.to_string());
                    let request_be = RequestBeResult{
                        ticketUuid : ticket_uuids.to_string(),
                        userUuid : data.user_id.to_string(),
                        rewardName : data.reward_name.to_string(),
                        rewardDescriptions:data.reward_description.to_string(),
                        status : "used".to_string(),
                        rewardType: select_enum_reward(data.reward_type.to_string()),
                        money : data.money
                    };
                    let post_request = post_to_be(request_be,data.url_address.to_string()).await;
                    if post_request {
                        let _update_used = diesel::update(tb_spin_used.filter(crate::adapters::spi::cfg::schema::tb_spin_used::dsl::ticket_uuid.eq(ticket_uuids.to_string()))).set((used_status.eq("success"),crate::adapters::spi::cfg::schema::tb_spin_used::dsl::updated_at.eq(SystemTime::now()))).execute(&mut CONN.get().unwrap().get().expect("failed connect db"));
                        let _update_failed = diesel::update(tb_spin_failed_process.filter(crate::adapters::spi::cfg::schema::tb_spin_failed_process::dsl::ticket_uuid.eq(ticket_uuids.to_string()))).set((post_status.eq("success"),crate::adapters::spi::cfg::schema::tb_spin_failed_process::dsl::updated_at.eq(SystemTime::now()))).execute(&mut CONN.get().unwrap().get().expect("failed connect db"));
                    }
        
                }
            }
}
