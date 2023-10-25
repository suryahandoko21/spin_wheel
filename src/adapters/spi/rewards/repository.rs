use super::mappers::{SpinRewardsActiveDbMapper, SpinRewardsDbMapper};
use super::models::{SpinRewardToDB, SpinRewardUpdateToDB, SpinRewards};
use super::status_active::status_active_spinwheel;
use crate::adapters::api::spin_reward::query_string::QstringReward;
use crate::adapters::api::spin_reward::spin_reward_payload::SpinRewardUpdatedPayload;
use crate::adapters::spi::cfg::pg_connection::CONN;
use crate::adapters::spi::cfg::schema::tb_spin_rewards::dsl::*;
use crate::application::mappers::db_mapper::DBMapper;
use crate::application::repositories::spin_company_repository_abstract::SpinCompanyEntityAbstract;
use crate::application::repositories::spin_ticket_repository_abstract::SpinTicketEntityAbstract;
use crate::domain::spin_reward_entity::{
    ActiveRewardEntity, SpinRewardActiveEntity, SpinRewardEntity,
};
use crate::{
    adapters::{
        api::{
            shared::{enum_response::Status, response::GenericResponse},
            spin_reward::spin_reward_payload::SpinRewardPayload,
        },
        spi::cfg::db_connection::ConnectionRepository,
    },
    application::repositories::spin_rewards_repository_abstract::SpinRewardEntityAbstract,
};
use async_trait::async_trait;
use diesel::dsl::*;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::error::Error;
use std::time::SystemTime;
#[async_trait(?Send)]
impl SpinRewardEntityAbstract for ConnectionRepository {
    async fn get_one_zonk_spin_reward_by_company(
        &self,
        company_code: String,
    ) -> Result<SpinRewardEntity, Box<dyn Error>> {
        let result = tb_spin_rewards
            .filter(reward_category.eq("NONE"))
            .filter(reward_status.eq("active"))
            .filter(companies_code.eq(company_code))
            .get_result::<SpinRewards>(
                &mut CONN.get().unwrap().get().expect("cant connect database"),
            );
        match result {
            Ok(models) => Ok(SpinRewardsDbMapper::to_entity(models)),
            Err(e) => Err(Box::new(e)),
        }
    }
    async fn get_one_spin_reward_by_id(
        &self,
        reward_id: i32,
    ) -> Result<SpinRewardEntity, Box<dyn Error>> {
        let result = tb_spin_rewards
            .filter(id.eq(reward_id))
            .get_result::<SpinRewards>(
                &mut CONN.get().unwrap().get().expect("cant connect database"),
            );
        match result {
            Ok(models) => Ok(SpinRewardsDbMapper::to_entity(models)),
            Err(e) => Err(Box::new(e)),
        }
    }
    async fn post_spin_rewards(
        &self,
        email: String,
        post: &SpinRewardPayload,
    ) -> Result<GenericResponse, Box<dyn Error>> {
        let data = &post;
        let company_code = &data.company_code;
        let mut messages = Status::PercentageMismatch.to_string();
        let mut statuses = Status::Fail.to_string();
        let mut total_percentage = 0.0;
        for spin in &data.detail {
            total_percentage += spin.percentage;
        }
        if total_percentage == 100.0 {
            let reward_exist = select(exists(
                tb_spin_rewards.filter(companies_code.eq(company_code)),
            ))
            .get_result::<bool>(&mut CONN.get().unwrap().get().expect("failed connect db"));
            if reward_exist.unwrap() {
                messages = Status::DataExist.to_string();
            } else {
                messages = Status::DataAdd.to_string();
                statuses = Status::Success.to_string();
                for spin in &data.detail {
                    let prepare_data = SpinRewardToDB {
                        companies_code: company_code.to_string(),
                        reward_category: spin.category.to_string(),
                        reward_image: spin.image.to_string(),
                        reward_name: spin.name.to_string(),
                        reward_note: spin.desc.to_string(),
                        reward_status: spin.status.to_string(),
                        reward_order: spin.order,
                        percentage: spin.percentage,
                        reward_amount: spin.amount,
                        reward_money: spin.money,
                        created_at: SystemTime::now(),
                        updated_at: SystemTime::now(),
                        created_by: email.to_string(),
                        updated_by: email.to_string(),
                    };
                    let value: Vec<SpinRewardToDB> = vec![prepare_data];
                    let _insert = diesel::insert_into(tb_spin_rewards)
                        .values(&value)
                        .execute(&mut CONN.get().unwrap().get().expect("cant connect database"));
                }
            }
        }
        Ok(GenericResponse {
            status: statuses,
            message: messages,
        })
    }

    async fn get_active_spin_reward_by_company_code(
        &self,
        company_code: String,
        user_uuid: String,
        is_login: bool,
    ) -> Result<SpinRewardActiveEntity, Box<dyn Error>> {
        let result_query = tb_spin_rewards
            .filter(reward_status.eq("active"))
            .filter(companies_code.eq(&company_code))
            .load::<SpinRewards>(&mut CONN.get().unwrap().get().expect("can't connect database"));
        let company =
            SpinCompanyEntityAbstract::get_spin_company_by_code(self, company_code.to_string())
                .await;
        let mut company_obj = SpinRewardActiveEntity {
            status: false,
            float_image:"".to_string(),
            user_uuid: "".to_string(),
            company_code: "".to_string(),
            reward_list: None,
            chance_spin: 0,
        };
        if !company.is_err() {
            let url_addresses = company.unwrap().companies_address.to_string();
            let (status_active,url_image) = status_active_spinwheel(url_addresses.to_string()).await;
            let c_spin =
                SpinTicketEntityAbstract::get_spin_ticket_by_uuid(self, user_uuid.to_string())
                    .await;
            let mut chance_spin = c_spin.ok().unwrap().spin_amount;
            if !is_login {
                chance_spin = 0;
            }
            /* Fill Struct Data */
            company_obj.status = status_active;
            company_obj.float_image = url_image;
            company_obj.user_uuid = user_uuid.to_string();
            company_obj.company_code = company_code.to_string();
            company_obj.reward_list = Some(
                result_query
                    .ok()
                    .unwrap()
                    .into_iter()
                    .map(SpinRewardsActiveDbMapper::to_entity)
                    .collect::<Vec<ActiveRewardEntity>>(),
            );
            company_obj.chance_spin = chance_spin
        }
        Ok(company_obj)
    }
    async fn get_all_spin_reward_by_company_code(
        &self,
        company_code: String,
        qstring: &QstringReward,
    ) -> Result<Vec<SpinRewardEntity>, Box<dyn Error>> {
        let mut result_query = tb_spin_rewards
            .into_boxed()
            .filter(companies_code.eq(company_code));
        let qstrings = &qstring;
        if qstrings.name != None {
            result_query =
                result_query.filter(reward_name.eq(qstrings.name.as_ref().unwrap().to_string()));
        }
        if qstrings.status != None {
            result_query =
                result_query.filter(reward_status.eq(qstrings.name.as_ref().unwrap().to_string()));
        }
        if qstrings.types != None {
            result_query = result_query
                .filter(reward_category.eq(qstrings.name.as_ref().unwrap().to_string()));
        }
        let results: Result<Vec<SpinRewards>, diesel::result::Error> = result_query
            .load::<SpinRewards>(&mut CONN.get().unwrap().get().expect("can't connect database"));
        match results {
            Ok(models) => Ok(models
                .into_iter()
                .map(SpinRewardsDbMapper::to_entity)
                .collect::<Vec<SpinRewardEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_all_spin_reward_by_company_code_by_status(
        &self,
        company_code: String,
    ) -> Result<Vec<SpinRewardEntity>, Box<dyn Error>> {
        let results: Result<Vec<SpinRewards>, diesel::result::Error> = tb_spin_rewards
            .filter(companies_code.eq(company_code))
            .filter(reward_status.eq("active"))
            .load::<SpinRewards>(&mut CONN.get().unwrap().get().expect("can't connect database"));
        match results {
            Ok(models) => Ok(models
                .into_iter()
                .map(SpinRewardsDbMapper::to_entity)
                .collect::<Vec<SpinRewardEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn update_spin_rewards(
        &self,
        email: String,
        post: &SpinRewardUpdatedPayload,
    ) -> Result<GenericResponse, Box<dyn Error>> {
        let data = &post;
        let company_code = &data.company_code;
        let mut statuses = Status::Fail.to_string();
        let mut messages = Status::PercentageMismatch.to_string();
        let mut total_percentage = 0.0;
        for spin in &data.detail {
            total_percentage += spin.percentage;
        }
        let reward_exist = select(exists(
            tb_spin_rewards.filter(companies_code.eq(company_code)),
        ))
        .get_result::<bool>(&mut CONN.get().unwrap().get().expect("failed connect db"));
        if !reward_exist.unwrap() {
            messages = Status::DataNotExist.to_string();
        } else {
            if total_percentage == 100.0 {
                statuses = Status::Success.to_string();
                messages = Status::DataUpdated.to_string();
                for spin in &data.detail {
                    if spin.id != 0 {
                        let _update = diesel::update(tb_spin_rewards.find(spin.id))
                            .set(&SpinRewardUpdateToDB {
                                companies_code: company_code.to_string(),
                                reward_category: spin.category.to_string(),
                                reward_image: spin.image.to_string(),
                                reward_name: spin.name.to_string(),
                                reward_note: spin.desc.to_string(),
                                reward_status: spin.status.to_string(),
                                reward_order: spin.order,
                                percentage: spin.percentage,
                                reward_amount: spin.amount,
                                reward_money: spin.money,
                                updated_at: SystemTime::now(),
                                updated_by: email.to_string(),
                            })
                            .execute(
                                &mut CONN.get().unwrap().get().expect("cant connect database"),
                            );
                    } else {
                        let prepare_data = SpinRewardToDB {
                            companies_code: company_code.to_string(),
                            reward_category: spin.category.to_string(),
                            reward_image: spin.image.to_string(),
                            reward_name: spin.name.to_string(),
                            reward_note: spin.desc.to_string(),
                            reward_status: spin.status.to_string(),
                            reward_order: spin.order,
                            percentage: spin.percentage,
                            reward_amount: spin.amount,
                            reward_money: spin.money,
                            created_at: SystemTime::now(),
                            updated_at: SystemTime::now(),
                            created_by: email.to_string(),
                            updated_by: email.to_string(),
                        };
                        let value = vec![prepare_data];
                        let _insert = diesel::insert_into(tb_spin_rewards).values(&value).execute(
                            &mut CONN.get().unwrap().get().expect("cant connect database"),
                        );
                    }
                }
            }
        }
        Ok(GenericResponse {
            status: statuses,
            message: messages,
        })
    }

    async fn used_one_spin_by_reward_id(&self, reward_id: i32) -> bool {
        let check_quantity = tb_spin_rewards
            .filter(id.eq(&reward_id))
            .select(reward_amount)
            .get_result::<i32>(&mut CONN.get().unwrap().get().expect("Failed connect database"));
        match check_quantity {
            Ok(val) => {
                if val == 0 {
                    return false;
                }
                let _update = diesel::update(tb_spin_rewards.filter(id.eq(reward_id)))
                    .filter(reward_category.ne("NONE"))
                    .set(reward_amount.eq(val - 1))
                    .execute(&mut CONN.get().unwrap().get().expect("Failed connect database"));
                return true;
            }
            Err(_) => {
                return false;
            }
        };
    }
}
