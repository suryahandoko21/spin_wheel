use async_trait::async_trait;
use diesel::{prelude::*};
use std::error::Error;
use std::mem;
use crate::adapters::api::shared::enum_response::Status;
use crate::adapters::api::shared::response::GenericResponse;
use crate::adapters::api::spin_promos::spin_promos_payloads::SpinPromosPayload;
// use std::mem;
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::spi::cfg::schema::{tb_spin_promos};
use crate::adapters::spi::cfg::{schema::tb_spin_promos::dsl::*};
use std::time::SystemTime;
use crate::adapters::api::shared::enum_response::Option;

use crate::application::repositories::spin_promos_repository_abstract::SpinPromosEntityAbstract;

use crate::application::{mappers::db_mapper::DBMapper};
use crate::domain::spin_promos_entity::SpinPromosEntity;

use super::mappers::SpinPromosDbMapper;
use super::models::{SpinPromos, SpinPromosToDb};

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Local};
use chrono::format::ParseError;


#[async_trait(?Send)]
impl SpinPromosEntityAbstract for ConnectionRepository {
    async fn get_all_spin_promos(&self) -> Result<Vec<SpinPromosEntity>, Box<dyn Error>>{
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let results = tb_spin_promos.load::<SpinPromos>(&mut conn);
        match results {
            Ok(models) => Ok(models.into_iter().map(SpinPromosDbMapper::to_entity).collect::<Vec<SpinPromosEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }
    async fn post_one_spin_promos(&self, post: &SpinPromosPayload) ->  Result<GenericResponse, Box<dyn Error>>{
        let mut data =  post.clone();
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let  prepare_data = SpinPromosToDb{
            promo_amount : mem::take(&mut data.promo_amount),
            promo_status: "not_used".to_string(),
            user_id : mem::take(&mut data.user_id),
            username: mem::take(&mut data.username),
            expired_at : Local::now().naive_local() ,
            point_currention_time :  Local::now().naive_local(),
            // NaiveTime::parse_from_str("23:56:04", "%H:%M:%S"),
            created_at :  Local::now().naive_local(), 
            updated_at :  Local::now().naive_local(),
            created_by : "System".to_string(),
            updated_by : "System".to_string(),
             
            };
            // println!()
        let to_vector = vec![prepare_data];
        let insert =   diesel::insert_into(tb_spin_promos).values(&to_vector).execute(&mut conn);
        match insert {
        Ok(_) => Ok(GenericResponse { status: Status::Success.to_string(),message:Option::Add.to_string()}),
        Err(e) => Err(Box::new(e)),   
        }
    }
}