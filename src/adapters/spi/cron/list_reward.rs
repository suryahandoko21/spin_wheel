use diesel::prelude::*;
use crate::adapters::api::shared::init_global::GLOBAL_INIT;
use crate::adapters::api::spin_reward::list_reward_mappers::ListRewardPresenterMapper;
use crate::adapters::api::spin_reward::spin_reward_presenters::ListRewardsPresenter;
use crate::adapters::spi::cfg::pg_connection::CONN;
use crate::adapters::spi::rewards::mappers::SpinRewardsDbMapper;
use crate::adapters::spi::rewards::models::SpinRewards;
use crate::adapters::spi::companies::models::Companies;
use crate::adapters::spi::cfg::schema::tb_spin_rewards::dsl::*;
use crate::adapters::spi::cfg::schema::tb_companies::dsl::*;
use crate::application::mappers::api_mapper::ApiMapper;
use crate::application::mappers::db_mapper::DBMapper;
use crate::domain::spin_reward_entity::SpinRewardEntity;
pub async fn check_list_reward(){
    let company  = tb_companies.load::<Companies>(&mut CONN.get().unwrap().get().expect("cant connect database"));
    for i in company.iter(){
        for c in i.iter(){
           let result_query:Result<Vec<SpinRewards>,_> = tb_spin_rewards.filter(reward_status.eq("active")).filter(crate::adapters::spi::cfg::schema::tb_spin_rewards::companies_code.eq(&c.companies_code)).load::<SpinRewards>(&mut CONN.get().unwrap().get().expect("cant connect database"));
           let entity = result_query.ok().unwrap().into_iter().map(SpinRewardsDbMapper::to_entity).collect::<Vec<SpinRewardEntity>>();
           let value =entity.into_iter().map(ListRewardPresenterMapper::to_api).collect::<Vec<ListRewardsPresenter>>();
           let url_address = &c.companies_address;
           let global_map = GLOBAL_INIT.get().unwrap();
           let url_prefix = "services/backend/api/spinwheel/callback/submit-rewards";
           let address = format!("{}/{}", url_address, url_prefix);
           let token_validation_be = &global_map["token_validation_be"]; 
           let client = awc::Client::default();
           let _response = client.post(address)
                    .insert_header(("spinWheelEngineSecretKey", token_validation_be.to_string()))
                    .send_json(&value).await;
        }
    }
   
}   