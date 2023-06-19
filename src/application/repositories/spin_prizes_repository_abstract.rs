use async_trait::async_trait;
// use crate::domain::sp::SpinPrizesEntity;
use crate::{domain::spin_prizes_entity::SpinPrizesEntity, adapters::api::{spin_prizes::spin_prizes_payloads::SpinPrizesPayload, shared::response::GenericResponse}};
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait SpinPrizesEntityAbstract {

    //  Spin Prize
    async fn get_one_spin_prize_by_id(&self, prize_id: i32) -> Result<SpinPrizesEntity, Box<dyn Error>>;
    async fn get_all_spin_prizes(&self) -> Result<Vec<SpinPrizesEntity>, Box<dyn Error>>;
    async fn post_one_spin_prize(&self, post: &SpinPrizesPayload) ->  Result<GenericResponse, Box<dyn Error>>;
    async fn delete_one_spin_prize_by_id(&self, prized_id: i32) ->  Result<GenericResponse, Box<dyn Error>>;
    async fn updated_one_spin_prize(&self, prize_id:i32,post: &SpinPrizesPayload) ->  Result<GenericResponse, Box<dyn Error>>;
}
