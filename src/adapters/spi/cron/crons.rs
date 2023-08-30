use async_trait::async_trait;
use crony::{Job, Runner, Schedule};
use std::thread;
use std::time::Duration;
use crate::adapters::spi::cfg::pg_connection::CONN;
struct QueueJob;
#[async_trait(?Send)]
impl Job for QueueJob {
    fn schedule(&self) -> Schedule {
        "1/15 * * * * *".parse().unwrap()
    }
    fn handle(&self) {

        let mut _con =CONN.get().unwrap().get().expect("cant connect");
        // println!("dsdsasda{:?}",con.get());
        // let query = tb_spin_tickets.filter(user_uuid.eq("14f17f89-ec80-4911-98a1-628ae2ca3e87")).select(count(id)).get_result::<i64>(&mut con);
        // println!("query{:?}",query);
        // CONN
        // let _f = <ConnectionRepository as CronEntityAbstract>::check_pending_post_be();
        // let c= CronEntityAbstract::check_pending_post_be(&self);
    }
}
pub fn cron_all(){
    let mut runner = Runner::new();
    runner = runner.add(Box::new(QueueJob));
    println!("Starting the Runner for 20 seconds");
    runner.run();
    thread::sleep(Duration::from_millis(20 * 1000));
}
