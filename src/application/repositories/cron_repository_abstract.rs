use async_trait::async_trait;
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

use crate::adapters::api::shared::response::GenericResponse;
#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait CronEntityAbstract {
    async fn check_pending_post_be(&self) -> Result<GenericResponse, Box<dyn Error>>;
}

pub struct Reader(String);
#[async_trait(?Send)]
pub trait Readable {
    async fn readable();
}

#[async_trait(?Send)]
impl Readable for Reader {
    async fn readable() {
        do_stuff().await
    }
}

async fn do_stuff() {
    println!("masuk");
}
