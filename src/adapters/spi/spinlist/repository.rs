use std::error::Error;

use async_trait::async_trait;
use diesel::associations::HasTable;
use diesel::{RunQueryDsl, QueryDsl, SelectableHelper};

use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::spi::cfg::{db_connection::DbConnection,schema::tb_spin_lists::dsl::*};
use  crate::adapters::spi::cfg::schema::tb_spin_prizes;
use crate::adapters::spi::prizes::models::{SpinPrizes, TestData};
use crate::application::{mappers::db_mapper::DBMapper,repositories::spin_lists_repository_abstract::SpinListsEntityAbstract};
use crate::domain::spin_lists_entity::SpinListsEntity;

use super::{models::SpinLists, mappers::SpinListDbMapper};

#[async_trait(?Send)]
impl SpinListsEntityAbstract for ConnectionRepository {
    async fn get_all_spin_lists(&self) -> Result<Vec<SpinListsEntity>, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let  _x = tb_spin_lists::table().inner_join(tb_spin_prizes::table).select(SpinPrizes::as_select()).get_result(&mut conn);
        println!("{}","===============================");
        println!("{}","===============================");
        println!("{}","===============================");
        println!("{}","===============================");
        println!("{}","===============================");
        let page_with_book = tb_spin_lists::table().inner_join(tb_spin_prizes::table).select((SpinLists::as_select(), SpinPrizes::as_select())).load::<(SpinLists, SpinPrizes)>(&mut conn).expect("sss");
        println!("cibay lah {:?}",page_with_book);
        // get_result(&mut conn)?;

        // let join = tb_spin_lists.inner_join(tb_spin_prizes::table)
        // .select((tb_spin_lists::dsl::))
        // .load::<TesData>(&mut conn);
        let results = tb_spin_lists.load::<SpinLists>(&mut conn);
        match results {
            Ok(models) => Ok(models.into_iter().map(SpinListDbMapper::to_entity).collect::<Vec<SpinListsEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }
}