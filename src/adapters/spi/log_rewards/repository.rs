use super::models::LogRewardsToDb;
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::spi::cfg::pg_connection::CONN;
use crate::adapters::spi::cfg::schema::tb_log_rewards::dsl::*;
use crate::application::repositories::log_reward_repository::LogRewardAbstract;
use async_trait::async_trait;
use diesel::RunQueryDsl;
use std::time::SystemTime;
#[async_trait(?Send)]
impl LogRewardAbstract for ConnectionRepository {
    async fn log_reward_actifity(
        &self,
        companies: String,
        created: String,
        before: String,
        after: String,
    ) {
        let prepare_data = LogRewardsToDb {
            companies_code: companies,
            reward_before: before,
            reward_after: after,
            created_at: SystemTime::now(),
            created_by: created,
        };
        let to_vector = vec![prepare_data];
        let _res = diesel::insert_into(tb_log_rewards)
            .values(&to_vector)
            .execute(&mut CONN.get().unwrap().get().expect("Failed connect database"));
    }
}
