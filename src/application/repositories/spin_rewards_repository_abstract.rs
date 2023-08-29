use async_trait::async_trait;
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

use crate::{adapters::api::{spin_reward::spin_reward_payload::{SpinRewardPayload, SpinRewardUpdatedPayload}, shared::response::GenericResponse}, domain::spin_reward_entity::SpinRewardEntity};
#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait SpinRewardEntityAbstract { 
    async fn post_spin_rewards(&self, post: &SpinRewardPayload) ->  Result<GenericResponse, Box<dyn Error>>;
    async fn get_all_spin_reward_by_company_code(&self,company_code: String) -> Result<Vec<SpinRewardEntity>, Box<dyn Error>>;
    async fn update_spin_rewards(&self, post: &SpinRewardUpdatedPayload) ->  Result<GenericResponse, Box<dyn Error>>;
    }
