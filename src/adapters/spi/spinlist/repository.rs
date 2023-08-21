use std::error::Error;
use std::fs::File;
use std::mem;
use std::time::SystemTime;

use async_trait::async_trait;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use diesel::sql_query;
use crate::adapters::api::shared::enum_response::Option;
use crate::adapters::api::shared::enum_response::Status;
use crate::adapters::api::shared::response::GenericResponse;
use crate::adapters::api::spin_lists::spin_list_payloads::{SpinListPayload, SpinPostPayload};
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::spi::cfg::{schema::tb_spin_lists::dsl::*};
use crate::adapters::spi::spinlist::mappers::SpinListsPrizesDBMapper;
use crate::adapters::spi::spinlist::models::SpinListsPrizes;
use crate::application::{mappers::db_mapper::DBMapper,repositories::spin_lists_repository_abstract::SpinListsEntityAbstract};
use crate::domain::spin_lists_entity::{SpinListsPrizesEntity};
use super::models::SpinListsToDb;


#[async_trait(?Send)]
impl SpinListsEntityAbstract for ConnectionRepository {
    async fn post_spin_by_uuid(&self, post: &SpinPostPayload) ->  Result<GenericResponse, Box<dyn Error>>{
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let list_spins = sql_query("SELECT tb_spin_prizes.*, company_code,list_status ,prize_name,percentage,roleid,created_at,created_by,updated_at,updated_by FROM tb_spin_lists  inner join tb_spin_prizes on tb_spin_lists.spin_prizes_id  =  tb_spin_prizes.id").load::<SpinListsPrizes>(&mut conn);
        // let x = list_spins.unwrap();
        // println!("aaa{:?}",x);
        // for i in list_spins?.map(_){
        //     println!("sds");
        // }
        let f = File::open("/");
        match f {
        Ok(_) => Ok(GenericResponse { status: Status::Success.to_string(),message:Option::Add.to_string()}),
        Err(e) => Err(Box::new(e)),   
        }
    }
    async fn get_all_spin_lists(&self) -> Result<Vec<SpinListsPrizesEntity>, Box<dyn Error>> {
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let list_spins = sql_query("SELECT tb_spin_prizes.*, company_code,list_status ,prize_name,percentage,roleid,created_at,created_by,updated_at,updated_by FROM tb_spin_lists  inner join tb_spin_prizes on tb_spin_lists.spin_prizes_id  =  tb_spin_prizes.id").load::<SpinListsPrizes>(&mut conn);
        match list_spins {
            Ok(models) => Ok(models.into_iter().map(SpinListsPrizesDBMapper::to_entity).collect::<Vec<SpinListsPrizesEntity>>()),
            Err(e) => Err(Box::new(e)),
        }
    }

    // async fn get_one_spin_list_by_id(&self,_list_id:i32)->Result<SpinListsPrizesEntity,Box<dyn Error>>{
    //     let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
    //     let result =sql_query("SELECT tb_spin_prizes.*, company_code,list_status ,prize_name,quantity,created_at,created_by,updated_at,updated_by 
    //     FROM tb_spin_lists  inner join tb_spin_prizes on tb_spin_lists.spin_prizes_id  =  tb_spin_prizes.id where tb_spin_lists.id=3").get_result::<SpinListsPrizes>(&mut conn);
    //     match  result
    //      {
    //         Ok(models) => Ok(SpinListsPrizesDBMapper::to_entity(models)),
    //         Err(e) => Err(Box::new(e)),
    //     }
    // }

    async fn post_one_spin_list(&self, post: &SpinListPayload) ->  Result<GenericResponse, Box<dyn Error>>{
        let mut data =  post.clone();
        let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
        let  _result = diesel::delete(tb_spin_lists.filter(company_code.eq(data.company_code.to_string()))).execute(&mut conn);     
        for spin in data.rule{
           
            let  prepare_data = SpinListsToDb{
                company_code : data.company_code.to_string(),
                list_status : data.list_status.to_string(),
                percentage:spin.percentage,
                roleid :spin.ruleid,
                created_at : SystemTime::now(), 
                updated_at : SystemTime::now(),
                created_by : "Sistem".to_string(),
                updated_by : "Sistem".to_string(),
                spin_prizes_id: 1,     
                };
            let to_vector = vec![prepare_data];
            let insert =   diesel::insert_into(tb_spin_lists).values(&to_vector).execute(&mut conn);
        }

        let f = File::open("/");
        match f {
        Ok(_) => Ok(GenericResponse { status: Status::Success.to_string(),message:Option::Add.to_string()}),
        Err(e) => Err(Box::new(e)),   
        }
    }

    // async fn delete_one_spin_list_by_id(&self, list_id: i32) ->  Result<GenericResponse, Box<dyn Error>>{
    //     let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool"); 
    //     let  result = diesel::delete(tb_spin_lists.filter(id.eq(list_id))).execute(&mut conn);           
    //     match  result
    //      {
    //         Ok(res) => Ok(GenericResponse { status: if res == 1 { Status::Success.to_string() } else { Status::Fail.to_string() },
    //                              message: if res == 1 { Option::Delete.to_string() } else { Option::NotFound.to_string() }}),
    //         Err(e) => Err(Box::new(e)),
    //     }
    // }

    // async fn updated_one_spin_list(&self, list_id:i32,post: &SpinListPayload) ->  Result<GenericResponse, Box<dyn Error>>{
    //     let mut data =  post.clone();
    //     let mut conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");
    //     let update = diesel::update(tb_spin_lists.find(list_id)).set(&SpinListsToDb{
    //                                                                 company_code : mem::take(&mut data.company_code),
    //                                                                 list_status : mem::take(&mut data.company_code),
    //                                                                 quantity: mem::take(&mut data.quantity),
    //                                                                 created_at : SystemTime::now(), 
    //                                                                 updated_at : SystemTime::now(),
    //                                                                 created_by : "Sistem".to_string(),
    //                                                                 updated_by : "Sistem".to_string(),
    //                                                                 spin_prizes_id:mem::take(&mut data.spin_prizes_id)
    //                                                                  }).execute(&mut conn);
    //     match update {
    //     Ok(_res) => Ok(GenericResponse { status: Status::Success.to_string(),message:Option::Update.to_string()}),
    //     Err(e) => Err(Box::new(e)),   
    //     }
    //     } 
    }
