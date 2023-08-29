use async_trait::async_trait;

#[cfg(test)]
use mockall::{predicate::*, *};


// #[cfg_attr(test, automock)]
// #[async_trait(?Send)]
pub trait CronEntityAbstract {
  fn check_pending_post_be(self);
  
}
