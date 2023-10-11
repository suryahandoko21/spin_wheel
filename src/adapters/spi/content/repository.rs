use std::error::Error;
use std::time::SystemTime;
use crate::adapters::api::content::content_payload::ContentPayload;
use crate::adapters::api::shared::enum_response::Status;
use crate::adapters::api::shared::response::GenericResponse;
use crate::{adapters::spi::cfg::pg_connection::CONN, application::mappers::db_mapper::DBMapper};
use async_trait::async_trait;
use diesel::dsl::exists;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, select};
use crate::{application::repositories::content_company_repository::ContentCompanyEntityAbstract, adapters::spi::cfg::db_connection::ConnectionRepository, domain::content_entity::ContentEntity};
use crate::adapters::spi::cfg::schema::tb_content::dsl::*;

use super::models::{ContentToDb, ContentUpdateDb};
use super::{models::Content, mappers::ContentDbMapper};

#[async_trait(?Send)]
impl ContentCompanyEntityAbstract for ConnectionRepository {
    async fn get_content_by_company_by_id(&self,company_code: String) -> Result<ContentEntity, Box<dyn Error>>{
        let result = tb_content.filter(companies_code.eq(company_code)).get_result::<Content>(&mut CONN.get().unwrap().get().expect("cant connect database"));
        match  result
         {
            Ok(models) => Ok(ContentDbMapper::to_entity(models)),
            Err(e) => Err(Box::new(e)),
        }

    }

    async fn post_contents(&self,company_code: String, post: &ContentPayload) ->  Result<GenericResponse, Box<dyn Error>>{
        let statuses;
        let  content_exist = select(exists(tb_content.filter(companies_code.eq(&company_code)))).get_result::<bool>(&mut CONN.get().unwrap().get().expect("failed connect db"));
        if  !content_exist.unwrap() {
            statuses = Status::DataAdd.to_string();
            let  prepare_data = ContentToDb{
                 companies_code:  company_code,
                 content_title: post.content_title.to_string(),
                 content_description: post.content_description.to_string(),
                 created_at: SystemTime::now(),
                 updated_at: SystemTime::now(),
                 created_by: "System".to_string(),
                 updated_by: "System".to_string()
                 };
                 let to_vector = vec![prepare_data]; 
                 let _ = diesel::insert_into(tb_content).values(&to_vector).execute(&mut CONN.get().unwrap().get().expect("Failed connect database"));       
                }else{
                    statuses = Status::DataUpdated.to_string();
                    let _update = diesel::update(tb_content.filter(companies_code.eq(&company_code.to_string()))).set(&ContentUpdateDb
                        { 
                            content_title: post.content_title.to_string(),
                            content_description: post.content_description.to_string(),
                            updated_at: SystemTime::now(),
                        }).execute(&mut CONN.get().unwrap().get().expect("cant connect database"));
              }
    let response =GenericResponse{ status: statuses, message: "Success".to_string() };
        Ok(response)
    }
}