use super::models::LogRewardsToDb;
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::spi::cfg::pg_connection::CONN;
use crate::adapters::spi::cfg::schema::tb_log_rewards::dsl::*;
use crate::application::repositories::log_reward_repository::LogRewardAbstract;
use crate::domain::log_reward_entity::{LogCustomRewardEntity, LogRewardEntity, UserEntity};
use async_trait::async_trait;
use chrono::format::StrftimeItems;
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::error::Error;
use std::time::SystemTime;

fn format_naive_datetime(naive_datetime: &NaiveDateTime, format: &str) -> String {
    let items = StrftimeItems::new(format);
    naive_datetime.format_with_items(items).to_string()
}

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
        company_code: String,
    ) -> Result<Vec<LogCustomRewardEntity>, Box<dyn Error>> {
        let results: Result<Vec<LogRewardEntity>, diesel::result::Error> = tb_log_rewards
            .filter(companies_code.eq(company_code))
            .load::<LogRewardEntity>(
                &mut CONN.get().unwrap().get().expect("can't connect database"),
            );
        let mut log_custom = LogCustomRewardEntity {
            id: 0,
            createdByUser: None,
            createdDate: "".to_string(),
            lastModifiedDate: "".to_string(),
            entityType: "SpinwheelReward".to_string(),
            valueBefore: "".to_string(),
            valueAfter: "".to_string(),
            value: "".to_string(),
            user: None,
            action: "".to_string(),
        };
        let mut output_log = Vec::new();
        for data_iter in results.into_iter() {
            for value in data_iter.into_iter() {
                let created_date_string =
                    format_naive_datetime(&value.created_at, "%Y-%m-%dT%H:%M:%S");
                let user = UserEntity {
                    username: value.created_by,
                };
                log_custom.id = value.id;
                log_custom.createdByUser = Some(user.to_owned());
                log_custom.createdDate = created_date_string.to_string();
                log_custom.lastModifiedDate = created_date_string.to_string();
                log_custom.valueBefore = value.reward_before;
                log_custom.valueAfter = value.reward_after;
                log_custom.value = value.reward_change;
                log_custom.user = Some(user.to_owned());
                log_custom.action = value.action_change;
                output_log.push(log_custom.clone());
            }
        }
        Ok(output_log)
    }
}
