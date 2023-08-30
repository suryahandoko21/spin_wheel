use async_trait::async_trait;
#[cfg(test)]
use mockall::{predicate::*, *};

use crate::adapters::spi::success::models::ProcessSuccessToDb;
#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait SuccessProcessEntityAbstract {
    async fn post_success_proccess(&self,value : ProcessSuccessToDb);
}