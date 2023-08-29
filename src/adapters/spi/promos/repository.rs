use actix_web::Result;
use async_trait::async_trait;
use diesel::dsl::exists;
use diesel::{prelude::*, select};

use std::error::Error;
use crate::adapters::api::shared::enum_response::Status;
use crate::adapters::api::shared::response::GenericResponse;

use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::spi::cfg::{schema::tb_spin_promos::dsl::*};
use crate::adapters::api::shared::enum_response::Option;
use crate::application::repositories::spin_promos_repository_abstract::SpinPromosEntityAbstract;
use crate::application::{mappers::db_mapper::DBMapper};
use crate::domain::spin_promos_entity::SpinPromosEntity;
use crate::helpers::fn_global::convert_str_to_timestamp;
use super::mappers::{SpinPromosDbMapper, SpinPromosApiMapper};
use super::models::{SpinPromos};

#[async_trait(?Send)]
impl SpinPromosEntityAbstract for ConnectionRepository {
    async fn get_all_spin_promos(&self) -> Result<Vec<SpinPromosEntity>, Box<dyn Error>>{
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let results = tb_spin_promos.load::<SpinPromos>(&mut conn);
        match results {
            Ok(models) => Ok(models.into_iter().map(SpinPromosApiMapper::to_entity).collect::<Vec<SpinPromosEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }
    async fn post_one_spin_promos(&self, post: &SpinPromosEntity) ->  Result<GenericResponse, Box<dyn Error>>{
        let entity_data =  post.clone();
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        // let rs= Err("hello");
        // let c = entity_data.point_currention_time;
        // let to_vector = vec![SpinPromosDbMapper::to_db(entity_data)];
        let  _point_exist = select(exists(tb_spin_promos.filter(point_currention_time.eq(convert_str_to_timestamp(entity_data.point_currention_time.to_string())))))
        .get_result::<bool>(&mut conn);
   
    // if point_exist.unwrap() {
    //     match r {
    //         Ok(x) => x,
    //         Err(e) => some_error(e),
    //     };
    // }
    //     match point_exist {
    //         Ok(_my_string) =>true,
    //         Err(some_error) => println!("{}", some_error),
    //         Ok(_my_string) =>true,
    //         Err(some_error) => println!("{}", some_error),
    //     }
    // }
    // else{
    //     println!("no oke cok");
    // }
        // Result

        let insert =   diesel::insert_into(tb_spin_promos).values(vec![SpinPromosDbMapper::to_db(entity_data)]).execute(&mut conn);
        match insert {
        Ok(_) => Ok(GenericResponse { status: Status::Success.to_string(),message:Option::Add.to_string()}),
        Err(e) => Err(Box::new(e)),   
        }
    }
}