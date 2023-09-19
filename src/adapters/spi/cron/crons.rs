use std::time::Duration;
use actix_rt::time;
use super::{pending_be::process_for_pending_be, check_expired::check_ticket_expired_be};
trait DurationExt {
    fn from_hours(hours: u64) -> Duration;
}

impl DurationExt for Duration {
    fn from_hours(hours: u64) -> Duration {
        Duration::from_secs(hours * 60 * 60)
    }
}
pub async fn job(){
    let mut interval = time::interval(Duration::from_secs(60));
    loop {
        interval.tick().await;
        process_for_pending_be().await;
        check_ticket_expired_be().await;
     }      
    } 
  
