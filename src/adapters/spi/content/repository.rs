use crate::adapters::api::content::content_payload::ContentPayload;
use crate::adapters::api::shared::enum_response::Status;
use crate::adapters::api::shared::response::GenericResponse;
use crate::adapters::spi::cfg::schema::tb_content::dsl::*;
use crate::application::repositories::log_repository::LogAbstract;
use crate::{
    adapters::spi::cfg::db_connection::ConnectionRepository,
    application::repositories::content_company_repository::ContentCompanyEntityAbstract,
    domain::content_entity::ContentEntity,
};
use crate::{adapters::spi::cfg::pg_connection::CONN, application::mappers::db_mapper::DBMapper};
use async_trait::async_trait;
use diesel::dsl::exists;
use diesel::{select, ExpressionMethods, QueryDsl, RunQueryDsl};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::time::SystemTime;

use super::models::{ContentToDb, ContentUpdateDb};
use super::{mappers::ContentDbMapper, models::Content};

fn compare_update(before: String, after: String) -> String {
    let parsed_json_after: Value = serde_json::from_str(&after).unwrap();
    let parsed_json_before: Value = serde_json::from_str(&before).unwrap();
    let title_b = parsed_json_before.get("content_title").unwrap();
    let desc_b = parsed_json_before.get("content_description").unwrap();
    let title_a = parsed_json_after.get("content_title").unwrap();
    let desc_a = parsed_json_after.get("content_description").unwrap();
    let mut map_update = HashMap::new();
    if title_b != title_a {
        map_update.insert(
            "title".to_string(),
            format!("{}=>{}", title_b.to_string(), title_a.to_string()),
        );
    }
    if desc_b != desc_a {
        map_update.insert(
            "description".to_string(),
            format!("{}=>{}", desc_b.to_string(), desc_a.to_string()),
        );
    }
    let serialized_update = serde_json::to_string(&map_update).unwrap();
    serialized_update
}
#[async_trait(?Send)]
impl ContentCompanyEntityAbstract for ConnectionRepository {
    async fn get_content_by_company_by_id(
        &self,
        company_code: String,
    ) -> Result<ContentEntity, Box<dyn Error>> {
        let result = tb_content
            .filter(companies_code.eq(company_code))
            .get_result::<Content>(&mut CONN.get().unwrap().get().expect("cant connect database"));
        match result {
            Ok(models) => Ok(ContentDbMapper::to_entity(models)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_content_default(&self) -> Result<ContentEntity, Box<dyn Error>> {
        let result = tb_content
            .filter(default.eq(true))
            .get_result::<Content>(&mut CONN.get().unwrap().get().expect("cant connect database"));
        match result {
            Ok(models) => Ok(ContentDbMapper::to_entity(models)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn post_contents(
        &self,
        company_code: String,
        email: String,
        ip: String,
        post: &ContentPayload,
    ) -> Result<GenericResponse, Box<dyn Error>> {
        let statuses;
        let content_exist = select(exists(tb_content.filter(companies_code.eq(&company_code))))
            .get_result::<bool>(&mut CONN.get().unwrap().get().expect("failed connect db"));
        if !content_exist.unwrap() {
            statuses = Status::DataAdd.to_string();
            let prepare_data = ContentToDb {
                companies_code: company_code.to_string(),
                content_title: post.content_title.to_string(),
                content_description: post.content_description.to_string(),
                created_at: SystemTime::now(),
                updated_at: SystemTime::now(),
                created_by: email.to_string(),
                updated_by: email.to_string(),
                default: false,
            };
            let to_vector = vec![prepare_data];
            let _ = diesel::insert_into(tb_content)
                .values(&to_vector)
                .execute(&mut CONN.get().unwrap().get().expect("Failed connect database"));
            let value = serde_json::to_string(&post).expect("Failed to serialize to JSON");
            let _log_reward = LogAbstract::log_actifity(
                self,
                (&company_code).to_string(),
                email.to_string(),
                "NONE".to_string(),
                value,
                "NONE".to_string(),
                "SpinwheelContent".to_string(),
                ip,
                "NEW DATA".to_string(),
            )
            .await;
        } else {
            statuses = Status::DataUpdated.to_string();
            let data_before = tb_content
                .filter(companies_code.eq(company_code.to_string()))
                .get_result::<Content>(
                    &mut CONN.get().unwrap().get().expect("cant connect database"),
                );
            let _update =
                diesel::update(tb_content.filter(companies_code.eq(&company_code.to_string())))
                    .set(&ContentUpdateDb {
                        content_title: post.content_title.to_string(),
                        content_description: post.content_description.to_string(),
                        updated_at: SystemTime::now(),
                        updated_by: email.to_string(),
                    })
                    .execute(&mut CONN.get().unwrap().get().expect("cant connect database"));
            let serde_before =
                serde_json::to_string(&data_before.unwrap()).expect("Failed to serialize to JSON");
            let serde_after = serde_json::to_string(&post).expect("Failed to serialize to JSON");
            let change = compare_update(serde_before.to_string(), serde_after.to_string());
            let _log_reward = LogAbstract::log_actifity(
                self,
                (&company_code).to_string(),
                email.to_string(),
                serde_before,
                serde_after.to_string(),
                change,
                "SpinwheelContent".to_string(),
                ip,
                "UPDATE".to_string(),
            )
            .await;
        }
        let response = GenericResponse {
            status: statuses,
            message: "Success".to_string(),
        };
        Ok(response)
    }
}
