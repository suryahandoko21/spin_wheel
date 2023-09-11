use std::time::SystemTime;
use std::error::Error;
use async_trait::async_trait;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use crate::adapters::api::spin_reward::spin_reward_payload::SpinRewardUpdatedPayload;
use crate::adapters::spi::cfg::pg_connection::CONN;
use crate::application::mappers::db_mapper::DBMapper;
use crate::domain::spin_reward_entity::SpinRewardEntity;
use crate::{application::repositories::spin_rewards_repository_abstract::SpinRewardEntityAbstract, adapters::{spi::cfg::db_connection::ConnectionRepository, api::{spin_reward::spin_reward_payload::SpinRewardPayload, shared::{response::GenericResponse, enum_response::Status}}}};
use crate::adapters::spi::cfg::schema::tb_spin_rewards::dsl::*;
use diesel::dsl::*;
use super::mappers::SpinRewardsDbMapper;
use super::models::{SpinRewardToDB, SpinRewards, SpinRewardUpdateToDB};
#[async_trait(?Send)]
impl SpinRewardEntityAbstract for ConnectionRepository {
    async fn get_one_spin_reward_by_id(&self,reward_id:i32)->Result<SpinRewardEntity,Box<dyn Error>>{
        let result = tb_spin_rewards.filter(id.eq(reward_id)).get_result::<SpinRewards>(&mut CONN.get().unwrap().get().expect("cant connect database"));
        match  result
         {
            Ok(models) => Ok(SpinRewardsDbMapper::to_entity(models)),
            Err(e) => Err(Box::new(e)),
        }
    }
    async fn post_spin_rewards(&self, post: &SpinRewardPayload) ->  Result<GenericResponse, Box<dyn Error>>{
       let data =  post.clone();
       let company_code = &data.company_code;
       let mut messages = Status::PercentageMismatch.to_string();
       let mut statuses =  Status::Fail.to_string();
       let mut total_percentage = 0;
       for spin in &data.detail{
             total_percentage += spin.percentage;
       }
       if total_percentage == 100{
        let  reward_exist = select(exists(tb_spin_rewards.filter(companies_code.eq(company_code)))).get_result::<bool>(&mut CONN.get().unwrap().get().expect("failed connect db"));
            if  reward_exist.unwrap() {
                messages = Status::DataExist.to_string();
            }
            else{
                messages = Status::DataAdd.to_string();
                statuses = Status::Success.to_string();
                for spin in data.detail{   
                    let prepare_data = SpinRewardToDB{
                        companies_code: company_code.to_string(),
                        reward_category:spin.category,
                        reward_image:spin.image,
                        reward_name:spin.name,
                        reward_note:spin.note,
                        reward_status:spin.status,
                        reward_order:spin.order,
                        percentage:spin.percentage,
                        reward_amount:spin.amount,
                        reward_money:spin.money,
                        created_at : SystemTime::now(), 
                        updated_at : SystemTime::now(),
                    };
                let value: Vec<SpinRewardToDB> = vec![prepare_data];
                let _insert = diesel::insert_into(tb_spin_rewards).values(&value).execute(&mut CONN.get().unwrap().get().expect("cant connect database"));
            }
        }
       }
        Ok(GenericResponse { status: statuses, message: messages})
    }

    async fn get_all_spin_reward_by_company_code(&self,company_code: String) -> Result<Vec<SpinRewardEntity>, Box<dyn Error>>{
        let results: Result<Vec<SpinRewards>, diesel::result::Error> = tb_spin_rewards.filter(companies_code.eq(company_code)).load::<SpinRewards>(&mut CONN.get().unwrap().get().expect("can't connect database"));
        match results {
            Ok(models) => Ok(models.into_iter().map(SpinRewardsDbMapper::to_entity).collect::<Vec<SpinRewardEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }   

     async fn get_all_spin_reward_by_company_code_by_status(&self,company_code: String) -> Result<Vec<SpinRewardEntity>, Box<dyn Error>>{
        let results: Result<Vec<SpinRewards>, diesel::result::Error> = tb_spin_rewards.filter(companies_code.eq(company_code)).filter(reward_status.eq("active")).load::<SpinRewards>(&mut CONN.get().unwrap().get().expect("can't connect database"));
        match results {
            Ok(models) => Ok(models.into_iter().map(SpinRewardsDbMapper::to_entity).collect::<Vec<SpinRewardEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }    

     async fn update_spin_rewards(&self, post: &SpinRewardUpdatedPayload) ->  Result<GenericResponse, Box<dyn Error>>{
        let  data =  post.clone();
        let company_code = &data.company_code;
        let mut statuses =  Status::Fail.to_string();
        let mut messages = Status::PercentageMismatch.to_string();
        let mut total_percentage = 0;
        for spin in &data.detail{
              total_percentage += spin.percentage;
        }
        let reward_exist = select(exists(tb_spin_rewards.filter(companies_code.eq(company_code)))).get_result::<bool>(&mut CONN.get().unwrap().get().expect("failed connect db"));
        if  !reward_exist.unwrap() {
            messages = Status::DataNotExist.to_string();
        }
        else{
            if total_percentage == 100{
                statuses = Status::Success.to_string();
                messages = Status::DataUpdated.to_string();
                for spin in data.detail{
                    if spin.id !=0{
                        let _update = diesel::update(tb_spin_rewards.find(spin.id)).set(&SpinRewardUpdateToDB{
                            companies_code:company_code.to_string(),
                            reward_category:spin.category,
                            reward_image:spin.image,
                            reward_name:spin.name,
                            reward_note:spin.note,
                            reward_status:spin.status,
                            reward_order:spin.order,
                            percentage:spin.percentage,
                            reward_amount:spin.amount,
                            reward_money:spin.money,
                            updated_at : SystemTime::now(),
                            }).execute(&mut CONN.get().unwrap().get().expect("cant connect database"));
                        }
                    else{
                        let prepare_data = SpinRewardToDB{
                            companies_code:company_code.to_string(),
                            reward_category:spin.category,
                            reward_image:spin.image,
                            reward_name:spin.name,
                            reward_note:spin.note,
                            reward_status:spin.status,
                            reward_order:spin.order,
                            percentage:spin.percentage,
                            reward_amount:spin.amount,
                            reward_money:spin.money,
                            created_at : SystemTime::now(), 
                            updated_at : SystemTime::now(),
                            };
                        let value = vec![prepare_data];
                        let _insert = diesel::insert_into(tb_spin_rewards).values(&value).execute(&mut CONN.get().unwrap().get().expect("cant connect database"));
                        }           
                    }
                }  
        }
        Ok(GenericResponse { status: statuses, message: messages})
     } 

  async fn used_one_spin_by_reward_id(&self, reward_id:i32) ->  bool{
        let check_quantity = tb_spin_rewards.filter(id.eq(&reward_id)).select(reward_amount).get_result::<i32>(&mut CONN.get().unwrap().get().expect("Failed connect database"));
        match check_quantity {
            Ok(val)=>{
                if val == 0{
                    return false;
                }
                let _update = diesel::update(tb_spin_rewards.filter(id.eq(reward_id))).set(reward_amount.eq(val-1)).execute(&mut CONN.get().unwrap().get().expect("Failed connect database"));
                return true;
            },
            Err(_)=>{
                return false;
            }
        };
        }   
}