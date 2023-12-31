use async_trait::async_trait;
#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

use crate::{
    adapters::api::{
        shared::response::GenericResponse,
        spin_reward::{
            query_string::QstringReward,
            spin_reward_payload::{SpinRewardPayload, SpinRewardUpdatedPayload},
        },
    },
    domain::spin_reward_entity::{SpinRewardActiveEntity, SpinRewardEntity},
};
#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait SpinRewardEntityAbstract {
    async fn get_one_spin_reward_by_id(
        &self,
        reward_id: i32,
    ) -> Result<SpinRewardEntity, Box<dyn Error>>;
    async fn get_one_zonk_spin_reward_by_company(
        &self,
        company_code: String,
    ) -> Result<SpinRewardEntity, Box<dyn Error>>;
    async fn post_spin_rewards(
        &self,
        email: String,
        remote_ip: String,
        post: &SpinRewardPayload,
    ) -> Result<GenericResponse, Box<dyn Error>>;
    async fn get_all_spin_reward_by_company_code(
        &self,
        company_code: String,
        qstring: &QstringReward,
    ) -> Result<Vec<SpinRewardEntity>, Box<dyn Error>>;
    async fn get_active_spin_reward_by_company_code(
        &self,
        company_code: String,
        user_uuid: String,
        is_login: bool,
    ) -> Result<SpinRewardActiveEntity, Box<dyn Error>>;
    async fn update_spin_rewards(
        &self,
        email: String,
        remote_ip: String,
        post: &SpinRewardUpdatedPayload,
    ) -> Result<GenericResponse, Box<dyn Error>>;
    async fn used_one_spin_by_reward_id(&self, prize_id: i32) -> bool;
    async fn get_all_spin_reward_by_company_code_by_status(
        &self,
        company_code: String,
    ) -> Result<Vec<SpinRewardEntity>, Box<dyn Error>>;
}
