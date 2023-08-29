
use crate::application::repositories::cron_repository_abstract::CronEntityAbstract;













use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
// use crate::application::repositories::cron_repository_abstract::CronEntityAbstract;
impl CronEntityAbstract for ConnectionRepository {
     fn check_pending_post_be(self){
        // let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool"); 
        // let c = self.db_connection.connect();
        // println!("DASDA{:?}",c);
        // let query = tb_spin_tickets.filter(user_uuid.eq("14f17f89-ec80-4911-98a1-628ae2ca3e87")).select(count(id)).get_result::<i64>(&mut c);
        // println!("query{:?}",query);
        // println!("check pending request->");
    }
}

