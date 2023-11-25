use super::models::LogsToDb;
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::spi::cfg::pg_connection::CONN;
use crate::adapters::spi::cfg::schema::tb_spin_logs::dsl::*;
use crate::application::repositories::log_repository::LogAbstract;
use crate::domain::log_entity::{LogCustomEntity, LogEntity, UserEntity};
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
impl LogAbstract for ConnectionRepository {
    async fn log_actifity(
        &self,
        companies: String,
        created: String,
        lbefore: String,
        lafter: String,
        lchange: String,
        entitytype: String,
        ip: String,
        action: String,
    ) {
        let prepare_data = LogsToDb {
            companies_code: companies,
            before: lbefore,
            after: lafter,
            change: lchange,
            remote_ip: ip,
            action_change: action,
            entity_type: entitytype,
            created_at: SystemTime::now(),
            created_by: created,
        };
        let to_vector = vec![prepare_data];
        let res = diesel::insert_into(tb_spin_logs)
            .values(&to_vector)
            .execute(&mut CONN.get().unwrap().get().expect("Failed connect database"));
        println!("ssssres{:?}", res);
    }

    async fn get_log_by_company_code(
        &self,
        company_code: String,
        etype: String,
    ) -> Result<Vec<LogCustomEntity>, Box<dyn Error>> {
        let results: Result<Vec<LogEntity>, diesel::result::Error> = tb_spin_logs
            .filter(companies_code.eq(company_code))
            .filter(entity_type.eq(etype))
            .load::<LogEntity>(&mut CONN.get().unwrap().get().expect("can't connect database"));
        let mut log_custom = LogCustomEntity {
            id: 0,
            createdByUser: None,
            createdDate: "".to_string(),
            lastModifiedDate: "".to_string(),
            entityType: "".to_string(),
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
                log_custom.valueBefore = value.before;
                log_custom.valueAfter = value.after;
                log_custom.value = value.change;
                log_custom.entityType = value.entity_type;
                log_custom.user = Some(user.to_owned());
                log_custom.action = value.action_change;
                output_log.push(log_custom.clone());
            }
        }
        Ok(output_log)
    }
}
