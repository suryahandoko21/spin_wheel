use async_trait::async_trait;
#[cfg(test)]
use mockall::{predicate::*, *};

use crate::adapters::spi::failed::models::FailedProcessToDb;
#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait FailedProcessEntityAbstract {
    async fn post_failed_proccess(&self, value: FailedProcessToDb);
}
