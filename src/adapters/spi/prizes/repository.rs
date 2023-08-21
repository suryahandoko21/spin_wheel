use async_trait::async_trait;
use diesel::{prelude::*};
use std::error::Error;
use std::mem;
use std::string;
use crate::adapters::api::shared::response::GenericResponse;
use crate::adapters::api::shared::enum_response::Status;
use crate::adapters::api::shared::enum_response::Option;
use crate::adapters::api::spin_prizes::spin_prizes_payloads::SpinPrizesPayload;
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::spi::cfg::schema::tb_spin_prizes::dsl::*;
use crate::adapters::spi::cfg::schema::tb_companies::dsl::*;
use crate::adapters::spi::companies::models::Companies;
use crate::adapters::spi::prizes::models::SpinPrizesToDB;
use crate::adapters::spi::prizes::{mappers::SpinPrizesDbMapper,models::SpinPrizes};
use crate::application::repositories::spin_company_repository_abstract::SpinCompanyEntityAbstract;
use crate::domain::spin_prizes_entity::SpinPrizesEntity;
use crate::application::{mappers::db_mapper::DBMapper,repositories::spin_prizes_repository_abstract::SpinPrizesEntityAbstract};

#[async_trait(?Send)]
impl SpinPrizesEntityAbstract for ConnectionRepository {
    async fn get_one_spin_prize_by_id(&self,prizes_id:i32)->Result<SpinPrizesEntity,Box<dyn Error>>{
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let result = tb_spin_prizes.filter(crate::adapters::spi::cfg::schema::tb_spin_prizes::dsl::id.eq(prizes_id)).get_result::<SpinPrizes>(&mut conn);
        match  result
         {
            Ok(models) => Ok(SpinPrizesDbMapper::to_entity(models)),
            Err(e) => Err(Box::new(e)),
        }
    }
    async fn get_all_spin_prizes_by_company_uuid(&self,company_uuid:String) -> Result<Vec<SpinPrizesEntity>, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let get_company = SpinCompanyEntityAbstract::get_spin_company_by_uuid(self, company_uuid).await;
        let mut company_id = 0;
        if get_company.as_ref().err().is_none(){
            company_id = get_company.as_ref().ok().unwrap().id;
        }
        let results: Result<Vec<SpinPrizes>, diesel::result::Error> = tb_spin_prizes.filter(companies_id.eq(company_id)).load::<SpinPrizes>(&mut conn);
        match results {
            Ok(models) => Ok(models.into_iter().map(SpinPrizesDbMapper::to_entity).collect::<Vec<SpinPrizesEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn post_one_spin_prize(&self,post: &SpinPrizesPayload)->Result<GenericResponse, Box<dyn Error> > {
        let mut data =  post.clone();
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let  prepare_data = SpinPrizesToDB{
                prize_name:mem::take(&mut data.prize_name),
                prize_note:mem::take(&mut data.prize_note),
                prize_category: mem::take(&mut data.prize_category),
                prize_amount:  mem::take(&mut data.prize_amount),
                percentage: mem::take(&mut data.percentage),
               
            };
        let to_vector = vec![prepare_data];
        let insert =   diesel::insert_into(tb_spin_prizes).values(&to_vector).execute(&mut conn);
        match insert {
        Ok(_) => Ok(GenericResponse { status: Status::Success.to_string(),message:Option::Add.to_string()}),
        Err(e) => Err(Box::new(e)),   
        }
        }

    async fn delete_one_spin_prize_by_id(&self,prize_id:i32)->Result<GenericResponse, Box<dyn Error> >{
            let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool"); 
            let  result = diesel::delete(tb_spin_prizes.filter(crate::adapters::spi::cfg::schema::tb_spin_prizes::dsl::id.eq(prize_id))).execute(&mut conn);           
            match  result
             {
                Ok(res) => Ok(GenericResponse { status: if res == 1 { Status::Success.to_string() } else { Status::Fail.to_string() },
                                     message: if res == 1 { Status::Fail.to_string() } else { Option::NotFound.to_string() }}),
                Err(e) => Err(Box::new(e)),
            }
        }

    async fn updated_one_spin_prize(&self,prize_id:i32, post: &SpinPrizesPayload) ->  Result<GenericResponse, Box<dyn Error>>{
        let mut data =  post.clone();
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let update = diesel::update(tb_spin_prizes.find(prize_id)).set(&SpinPrizesToDB{
                                    prize_name:mem::take(&mut data.prize_name),
                                    prize_note:mem::take(&mut data.prize_note),
                                    prize_category: mem::take(&mut data.prize_category),
                                    prize_amount:  mem::take(&mut data.prize_amount),
                                    percentage:mem::take(&mut data.percentage),
    }).execute(&mut conn);
        match update {
        Ok(_res) => Ok(GenericResponse { status: Status::Success.to_string(),message:Option::Update.to_string()}),
        Err(e) => Err(Box::new(e)),   
        }
        } 
     }