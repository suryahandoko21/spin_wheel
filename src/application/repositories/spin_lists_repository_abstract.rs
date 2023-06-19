use async_trait::async_trait;
// use crate::domain::sp::SpinPrizesEntity;
use crate::{domain::{ spin_lists_entity::SpinListsEntity}};
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait SpinListsEntityAbstract {
    // async fn get_one_spin_prize_by_id(&self, prize_id: i32) -> Result<SpinListsEntity, Box<dyn Error>>;
    async fn get_all_spin_lists(&self) -> Result<Vec<SpinListsEntity>, Box<dyn Error>>;
}