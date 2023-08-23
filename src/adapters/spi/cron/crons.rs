use async_trait::async_trait;
use crony::{Job, Runner, Schedule};
use std::thread;
use std::time::Duration;
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::application::repositories::cron_repository_abstract::CronEntityAbstract;
struct QueueJob;
#[async_trait(?Send)]
impl Job for QueueJob {
    fn schedule(&self) -> Schedule {
        "1/5 * * * * *".parse().unwrap()
    }
    fn handle(&self) {
        // let _f = <ConnectionRepository as CronEntityAbstract>::check_pending_post_be();
        // let c= CronEntityAbstract::check_pending_post_be(&self);
    }
}
pub fn cron_all(){
    let mut runner = Runner::new();
    runner = runner.add(Box::new(QueueJob));
    println!("Starting the Runner for 20 seconds");
    runner = runner.run();
    thread::sleep(Duration::from_millis(20 * 1000));
}
