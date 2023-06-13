use async_trait::async_trait;
use diesel::{prelude::*};
// use serde_json::Value;
use std::error::Error;
use std::mem;

use std::sync::Arc;

use crate::adapters::api::shared::response::GenericResponse;
use crate::adapters::api::spin_prizes::spin_prizes_payloads::SpinPrizesPayload;
use crate::adapters::spi::cfg::{db_connection::DbConnection,schema::tb_spin_prizes::dsl::*};
use crate::adapters::spi::prizes::models::SpinPrizesToDB;
use crate::adapters::spi::prizes::{mappers::SpinPrizesDbMapper,models::SpinPrizes};
use crate::domain::spin_prizes_entity::SpinPrizesEntity;
use crate::application::{mappers::db_mapper::DBMapper,repositories::spin_prizes_repository_abstract::SpinPrizesEntityAbstract};

pub struct SpinPrizesRepository {
    pub db_connection: DbConnection,
}


#[async_trait(?Send)]
impl SpinPrizesEntityAbstract for SpinPrizesRepository {


    async fn get_all_spin_prizes(&self) -> Result<Vec<SpinPrizesEntity>, Box<dyn Error>> {

        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let results = tb_spin_prizes.load::<SpinPrizes>(&mut conn);
        match results {
            Ok(models) => Ok(models.into_iter().map(SpinPrizesDbMapper::to_entity).collect::<Vec<SpinPrizesEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }
    async fn get_one_spin_prize_by_id(&self,prizes_id:i32)->Result<SpinPrizesEntity,Box<dyn Error>>{
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let result = tb_spin_prizes.filter(id.eq(prizes_id)).get_result::<SpinPrizes>(&mut conn);
        match  result
         {
            Ok(models) => Ok(SpinPrizesDbMapper::to_entity(models)),
            Err(e) => Err(Box::new(e)),
        }
    }
 
    
    async fn post_one_spin_prize(&self,post: &SpinPrizesPayload)->Result<GenericResponse, Box<dyn Error> > {
    //    let *f  = post;

        let mut data =  post.clone();
        let  foo = Arc::new(post).clone();
        println!("sdsad {}",foo.prize_category);
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
      
        let mut value = post;

        let  to_vector = SpinPrizesToDB{
                // id:1,
                prize_weight: post.prize_weight,
                prize_name:mem::take(&mut data.prize_name),
                prize_note:mem::take(&mut data.prize_note),
                prize_category: mem::take(&mut data.prize_category),
                prize_amount:  mem::take(&mut data.prize_amount)
               
            };
        let insert_data = vec![to_vector];
        
            
      let insert =   diesel::insert_into(tb_spin_prizes).values(&insert_data).execute(&mut conn);
      println!("hasil{:?}",insert);
      match insert {
        Ok(_res) => Ok(GenericResponse { status: "SUCCESS".to_string()}),
        Err(e) => Err(Box::new(e)),
          
      }

    //   let response_json: &GenericResponse = &GenericResponse {
    //     status: "success".to_string(),
    //     message: "sss".to_string(),
    // };
    //   assert_eq!(Ok(1), x);
// println!("result {:?}",x);
    //     // // .get_result(&mut conn)
    //     // .expect("Error saving new post");
    //     let inserted_users = insert_into(tb_spin_prizes)
    //     .values(post)
    //     .get_results::<SpinPrizes>(&mut conn);
        //     println!("mahagitututut {}" , post.prize_amount);
        // let results = tb_spin_prizes.load::<SpinPrizes>(&mut conn);
//  match x {
//     Ok(data)=>data,
//     Err(e) => Err(Box::new(e)),
//  }
// Ok(x);
        // match x {
        //     Ok(res) =>Ok(res),
        //       Err(e) => Err(Box::new(e)),
        // }


        }
        // post_one_spin_prize

}