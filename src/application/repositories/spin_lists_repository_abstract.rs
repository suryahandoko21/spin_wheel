use async_trait::async_trait;
// use crate::domain::sp::SpinPrizesEntity;
use crate::{domain::{ spin_lists_entity::{ SpinListsPrizesEntity}}, adapters::api::{shared::response::GenericResponse, spin_lists::spin_list_payloads::{SpinListPayload, SpinPostPayload}}};
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait SpinListsEntityAbstract {
    // // async fn get_one_spin_prize_by_id(&self, prize_id: i32) -> Result<SpinListsEntity, Box<dyn Error>>;
    async fn get_all_spin_lists(&self) -> Result<Vec<SpinListsPrizesEntity>, Box<dyn Error>>;
    // async fn get_one_spin_list_by_id(&self, lidt_id: i32) -> Result<SpinListsPrizesEntity, Box<dyn Error>>;
    async fn post_one_spin_list(&self, post: &SpinListPayload) ->  Result<GenericResponse, Box<dyn Error>>;

    async fn post_spin_by_uuid(&self, post: &SpinPostPayload) ->  Result<GenericResponse, Box<dyn Error>>;
    // async fn delete_one_spin_list_by_id(&self, list_id: i32) ->  Result<GenericResponse, Box<dyn Error>>;
    // async fn updated_one_spin_list(&self, list_id:i32,post: &SpinListPayload) ->  Result<GenericResponse, Box<dyn Error>>;
}