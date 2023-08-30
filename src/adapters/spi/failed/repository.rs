use async_trait::async_trait;
use diesel::RunQueryDsl;
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::spi::cfg::pg_connection::CONN;
use crate::adapters::spi::cfg::schema::tb_spin_failed_process::dsl::*;
use crate::application::repositories::failed_proccess_abstract::FailedProcessEntityAbstract;
use super::models::FailedProcessToDb;
#[async_trait(?Send)]
impl FailedProcessEntityAbstract for ConnectionRepository {
    async fn post_failed_proccess(&self,value : FailedProcessToDb){
        let data = value.clone();
        let to_vector = vec![data];
        let _insert = diesel::insert_into(tb_spin_failed_process).values(&to_vector).execute(&mut CONN.get().unwrap().get().expect("failed connect db"));
    }
}