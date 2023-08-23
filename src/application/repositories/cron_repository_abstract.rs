use async_trait::async_trait;

use crate::{domain::{ spin_lists_entity::{ SpinListsPrizesEntity}, spin_prizes_entity::SpinPrizesEntity, spin_company_entity::SpinCompanyEntity}, adapters::api::{shared::response::GenericResponse, spin_lists::spin_list_payloads::{SpinListPayload, SpinPostPayload}}};
#[cfg(test)]
use mockall::{predicate::*, *};


// #[cfg_attr(test, automock)]
// #[async_trait(?Send)]
pub trait CronEntityAbstract {
  fn check_pending_post_be(self);
  
}
