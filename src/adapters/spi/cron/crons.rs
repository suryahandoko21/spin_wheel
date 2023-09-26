use std::time::Duration;
use actix_rt::time;
use super::{pending_be::process_for_pending_be, check_expired::check_ticket_expired_be, list_reward::check_list_reward};
trait DurationExt {
    fn from_hours(hours: u64) -> Duration;
}

impl DurationExt for Duration {
    fn from_hours(hours: u64) -> Duration {
        Duration::from_secs(hours * 60 * 60)
    }
}
pub async fn perseconds(){
    let mut interval = time::interval(Duration::from_secs(60));
    loop {
        interval.tick().await;
        process_for_pending_be().await;
        check_ticket_expired_be().await;
     }      
    } 
  
pub async fn perdays(){
    let mut interval_day = time::interval(Duration::from_secs(Duration::new(24 * 60 * 60, 0).as_secs()));
        loop {
            interval_day.tick().await;
            check_list_reward().await;
        }   
}  
