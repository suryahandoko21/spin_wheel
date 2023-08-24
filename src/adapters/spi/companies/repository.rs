use async_trait::async_trait;
use diesel::prelude::*;
use crate::adapters::spi::cfg::schema::tb_companies::dsl::*;
use std::error::Error;
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::domain::spin_company_entity::SpinCompanyEntity;
use crate::application::{mappers::db_mapper::DBMapper,repositories::spin_company_repository_abstract::SpinCompanyEntityAbstract};
use super::mappers::SpinCompaniesDbMapper;
use super::models::Companies;
#[async_trait(?Send)]
impl SpinCompanyEntityAbstract for ConnectionRepository {
    async fn get_spin_company_by_id(&self,company_id: i32) -> Result<SpinCompanyEntity, Box<dyn Error>>{
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let result = tb_companies.filter(id.eq(company_id)).get_result::<Companies>(&mut conn);
        match  result
         {
            Ok(models) => Ok(SpinCompaniesDbMapper::to_entity(models)),
            Err(e) => Err(Box::new(e)),
        }
    }
    async fn get_spin_company_by_uuid(&self,uuids: String) -> Result<SpinCompanyEntity, Box<dyn Error>>{
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let result = tb_companies.filter(uuid.eq(uuids)).get_result::<Companies>(&mut conn);
        match  result
         {
            Ok(models) => Ok(SpinCompaniesDbMapper::to_entity(models)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_spin_company_by_code(&self,company_code: String) -> Result<SpinCompanyEntity, Box<dyn Error>>{
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let result = tb_companies.filter(companies_code.eq(company_code)).get_result::<Companies>(&mut conn);
        match  result
         {
            Ok(models) => Ok(SpinCompaniesDbMapper::to_entity(models)),
            Err(e) => Err(Box::new(e)),
        }
    }
    async fn fetch_spin_company_from_url(&self) ->bool{
        
        return true;
    }
     }