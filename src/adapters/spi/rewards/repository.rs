use super::mappers::{SpinRewardsActiveDbMapper, SpinRewardsDbMapper};
use super::models::{SpinRewardToDB, SpinRewardUpdateToDB, SpinRewards};
use super::status_active::status_active_spinwheel;
use crate::adapters::api::spin_reward::query_string::QstringReward;
use crate::adapters::api::spin_reward::spin_reward_payload::SpinRewardUpdatedPayload;
use crate::adapters::spi::cfg::pg_connection::CONN;
use crate::adapters::spi::cfg::schema::tb_spin_rewards::dsl::*;
use crate::application::mappers::db_mapper::DBMapper;
use crate::application::repositories::log_repository::LogAbstract;
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
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::time::SystemTime;

/* function to get compare before after change reward  */
fn compare_update(before: String, after: String) -> String {
    let parsed_json_after: Value = serde_json::from_str(&after).unwrap();
    let parsed_json_before: Value = serde_json::from_str(&before).unwrap();
    let id_x = parsed_json_after.get("id").unwrap();
    let name_x = parsed_json_after.get("name").unwrap();
    let note_x = parsed_json_after.get("desc").unwrap();
    let category_x = parsed_json_after.get("category").unwrap();
    let amount_x = parsed_json_after.get("amount").unwrap();
    let money_x = parsed_json_after.get("money").unwrap();
    let percentage_x = parsed_json_after.get("percentage").unwrap();
    let image_x = parsed_json_after.get("image").unwrap();
    let status_x = parsed_json_after.get("status").unwrap();
    let order_x = parsed_json_after.get("order").unwrap();
    if let Some(arr) = parsed_json_before.as_array() {
        let mut map_update = HashMap::new();
        for obj in arr {
            let id_o = obj.get("id").unwrap();
            let name_o = obj.get("reward_name").unwrap();
            let note_o = obj.get("reward_note").unwrap();
            let category_o = obj.get("reward_category").unwrap();
            let amount_o = obj.get("reward_amount").unwrap();
            let money_o = obj.get("reward_money").unwrap();
            let percentage_o = obj.get("percentage").unwrap();
            let image_o = obj.get("reward_image").unwrap();
            let status_o = obj.get("reward_status").unwrap();
            let order_o = obj.get("reward_order").unwrap();
            if id_o == id_x {
                if name_o != name_x {
                    map_update.insert(
                        "name".to_string(),
                        format!("{}=>{}", name_o.to_string(), name_x.to_string()),
                    );
                }
                if note_o != note_x {
                    map_update.insert(
                        "description".to_string(),
                        format!("{}=>{}", note_o.to_string(), note_x.to_string()),
                    );
                }
                if category_o != category_x {
                    map_update.insert(
                        "category".to_string(),
                        format!("{}=>{}", category_o.to_string(), category_x.to_string()),
                    );
                }
                if amount_o != amount_x {
                    map_update.insert(
                        "amount".to_string(),
                        format!("{}=>{}", amount_o.to_string(), amount_x.to_string()),
                    );
                }
                if money_o != money_x {
                    map_update.insert(
                        "money".to_string(),
                        format!("{}=>{}", money_o.to_string(), money_x.to_string()),
                    );
                }
                if percentage_o != percentage_x {
                    map_update.insert(
                        "percentage".to_string(),
                        format!("{}=>{}", percentage_o.to_string(), percentage_x.to_string()),
                    );
                }
                if image_o != image_x {
                    map_update.insert(
                        "image".to_string(),
                        format!("{}=>{}", image_o.to_string(), image_x.to_string()),
                    );
                }
                if status_o != status_x {
                    map_update.insert(
                        "status".to_string(),
                        format!("{}=>{}", status_o.to_string(), status_x.to_string()),
                    );
                }
                if order_o != order_x {
                    map_update.insert(
                        "order".to_string(),
                        format!("{}=>{}", order_o.to_string(), order_x.to_string()),
                    );
                }
            }
        }
        if map_update.len() < 1 {
            map_update.clear();
        }
        let serialized_update = serde_json::to_string(&map_update).unwrap();
        return serialized_update;
    }
    "".to_string()
}

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
        remote_ip: String,
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
                let value =
                    serde_json::to_string(&data.detail).expect("Failed to serialize to JSON");
                let _log_reward = LogAbstract::log_actifity(
                    self,
                    (&company_code).to_string(),
                    email.to_string(),
                    "NONE".to_string(),
                    value,
                    "NONE".to_string(),
                    "SpinwheelReward".to_string(),
                    remote_ip,
                    "NEW DATA".to_string(),
                )
                .await;
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
            float_image: "".to_string(),
            user_uuid: "".to_string(),
            company_code: "".to_string(),
            reward_list: None,
            chance_spin: 0,
        };
        if !company.is_err() {
            let url_addresses = company.unwrap().companies_address.to_string();
            let (status_active, url_image) =
                status_active_spinwheel(url_addresses.to_string()).await;
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
        remote_ip: String,
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
                let mut map_change = HashMap::new();
                let mut map_insert = HashMap::new();
                let mut numb = 0;
                for spin in &data.detail {
                    if spin.id != 0 {
                        let result_query = tb_spin_rewards
                            .filter(companies_code.eq(&company_code))
                            .load::<SpinRewards>(
                                &mut CONN.get().unwrap().get().expect("can't connect database"),
                            );
                        let before =
                            serde_json::to_string(&result_query.as_ref().unwrap()).expect("Failed");
                        let after =
                            serde_json::to_string(spin).expect("Failed to serialize to JSON");
                        let compare = compare_update(before, after);
                        if compare.len() > 2 {
                            map_change.insert(format!("{}{}", "update_id:", spin.id), compare);
                        }

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
                        numb += 1;
                        let insert_change_value = serde_json::to_string(&spin).expect("Failed");
                        map_insert.insert(
                            format!("{}{}", "insert ".to_string(), numb),
                            insert_change_value,
                        );
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
                map_change.extend(map_insert);
                let result_query = tb_spin_rewards
                    .filter(companies_code.eq(&company_code))
                    .load::<SpinRewards>(
                    &mut CONN.get().unwrap().get().expect("can't connect database"),
                );
                let before = serde_json::to_string(&result_query.unwrap())
                    .expect("Failed to serialize to JSON");
                let after =
                    serde_json::to_string(&post.detail).expect("Failed to serialize to JSON");
                let change = serde_json::to_string(&map_change).expect("Failed");
                let _log_reward = LogAbstract::log_actifity(
                    self,
                    (&company_code).to_string(),
                    email.to_string(),
                    before,
                    after,
                    change,
                    "SpinwheelReward".to_string(),
                    remote_ip,
                    "UPDATE".to_string(),
                )
                .await;
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
