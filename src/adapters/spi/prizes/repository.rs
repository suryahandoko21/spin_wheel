use async_trait::async_trait;
use diesel::{prelude::*, insert_into};
use std::error::Error;


use crate::adapters::api::spin_prizes::spin_prizes_payloads::SpinPrizesPayload;
use crate::adapters::spi::cfg::{db_connection::DbConnection,schema::tb_spin_prizes::dsl::*};
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

    async fn post_one_spin_prize(&self,post: &SpinPrizesPayload)->Result<Vec<SpinPrizesEntity>, Box<dyn Error>> {
      
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        // let changed_animal = tb_spin_prizes.sa
        // println!("data mcb !!! {:?}",post.into_inner());
        // let new_post = new SpinPrizes{
        //     prize_amount: &post.
        // }
        //     post.prize_amount
        // };
        // diesel::insert_into(tb_spin_prizes).values(&post).returning(SpinPrizes::as_returning())
        // .get_result(&mut conn)
        // .expect("Error saving new post");
        // let inserted_users = insert_into(tb_spin_prizes)
        // .values(post)
        // .get_results::<SpinPrizes>(&mut conn);
        //     println!("mahagitututut {}" , post.prize_amount);
        let results = tb_spin_prizes.load::<SpinPrizes>(&mut conn);

        match results {
            Ok(models) => Ok(models.into_iter().map(SpinPrizesDbMapper::to_entity).collect::<Vec<SpinPrizesEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
        }
        // post_one_spin_prize

}