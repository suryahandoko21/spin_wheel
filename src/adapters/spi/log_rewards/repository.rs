use super::models::LogRewardsToDb;
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::spi::cfg::pg_connection::CONN;
use crate::adapters::spi::cfg::schema::tb_log_rewards::dsl::*;
use crate::application::repositories::log_reward_repository::LogRewardAbstract;
use crate::domain::log_reward_entity::LogRewardEntity;
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::error::Error;
use std::time::SystemTime;
#[async_trait(?Send)]
impl LogRewardAbstract for ConnectionRepository {
    async fn log_reward_actifity(
        &self,
        companies: String,
        created: String,
        before: String,
        after: String,
        change: String,
        ip: String,
        action: String,
    ) {
        let prepare_data = LogRewardsToDb {
            companies_code: companies,
            reward_before: before,
            reward_after: after,
            reward_change: change,
            remote_ip: ip,
            action_change: action,
            created_at: SystemTime::now(),
            created_by: created,
        };
        let to_vector = vec![prepare_data];
        let _res = diesel::insert_into(tb_log_rewards)
            .values(&to_vector)
            .execute(&mut CONN.get().unwrap().get().expect("Failed connect database"));
    }

    async fn get_log_reward_by_company_code(
        &self,
        _company_code: String,
    ) -> Result<Vec<LogRewardEntity>, Box<dyn Error>> {
        let results: Result<Vec<LogRewardEntity>, diesel::result::Error> = tb_log_rewards
            .filter(companies_code.eq(&companies_code))
            .load::<LogRewardEntity>(
                &mut CONN.get().unwrap().get().expect("can't connect database"),
            );
        Ok(results.unwrap())
    }
}
