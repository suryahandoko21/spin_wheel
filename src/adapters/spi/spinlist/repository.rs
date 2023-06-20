use std::error::Error;

use async_trait::async_trait;
use diesel::associations::HasTable;
use diesel::{RunQueryDsl, QueryDsl, SelectableHelper};
use diesel::sql_query;
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::spi::cfg::{schema::tb_spin_lists::dsl::*};
use  crate::adapters::spi::cfg::schema::tb_spin_prizes;
use crate::adapters::spi::prizes::mappers::SpinPrizesDbMapper;
use crate::adapters::spi::prizes::models::{SpinPrizes};
use crate::adapters::spi::spinlist::mappers::SpinListsPrizesDBMapper;
use crate::adapters::spi::spinlist::models::SpinListsPrizes;
use crate::application::{mappers::db_mapper::DBMapper,repositories::spin_lists_repository_abstract::SpinListsEntityAbstract};
use crate::domain::spin_lists_entity::{SpinListsEntity, SpinListsPrizesEntity};
use crate::domain::spin_prizes_entity::SpinPrizesEntity;
use super::{models::SpinLists, mappers::SpinListDbMapper};

#[async_trait(?Send)]
impl SpinListsEntityAbstract for ConnectionRepository {
    async fn get_all_spin_lists(&self) -> Result<Vec<SpinListsPrizesEntity>, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
       
        let list_prizes = sql_query("SELECT tb_spin_prizes.*, company_code,list_status ,prize_name,quantity,created_at,created_by,updated_at,updated_by FROM tb_spin_lists  inner join tb_spin_prizes on tb_spin_lists.spin_prizes_id  =  tb_spin_prizes.id").load::<SpinListsPrizes>(&mut conn);
        println!("----{:?}",list_prizes);
        let results = tb_spin_lists.load::<SpinLists>(&mut conn);

        match list_prizes {
            Ok(models) => Ok(models.into_iter().map(SpinListsPrizesDBMapper::to_entity).collect::<Vec<SpinListsPrizesEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get_one_spin_list_by_id(&self,_list_id:i32)->Result<SpinListsPrizesEntity,Box<dyn Error>>{
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let result =sql_query("SELECT tb_spin_prizes.*, company_code,list_status ,prize_name,quantity,created_at,created_by,updated_at,updated_by 
        FROM tb_spin_lists  inner join tb_spin_prizes on tb_spin_lists.spin_prizes_id  =  tb_spin_prizes.id where tb_spin_lists.id=3").get_result::<SpinListsPrizes>(&mut conn);
        match  result
         {
            Ok(models) => Ok(SpinListsPrizesDBMapper::to_entity(models)),
            Err(e) => Err(Box::new(e)),
        }
    }
}   