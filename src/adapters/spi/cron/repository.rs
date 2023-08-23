use diesel::{prelude::*};
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::application::repositories::cron_repository_abstract::CronEntityAbstract;
impl CronEntityAbstract for ConnectionRepository {
     fn check_pending_post_be(self){
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool"); 
        
        // println!("check pending request->");
    }
}

