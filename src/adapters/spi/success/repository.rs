use super::models::ProcessSuccessToDb;
use crate::adapters::spi::cfg::db_connection::ConnectionRepository;
use crate::adapters::spi::cfg::pg_connection::CONN;
use crate::adapters::spi::cfg::schema::tb_spin_success_process::dsl::*;
use crate::application::repositories::success_process_abstract::SuccessProcessEntityAbstract;
use async_trait::async_trait;
use diesel::RunQueryDsl;
#[async_trait(?Send)]
impl SuccessProcessEntityAbstract for ConnectionRepository {
    async fn post_success_proccess(&self, value: ProcessSuccessToDb) {
        let data = &value;
        let to_vector = vec![data];
        let _insert = diesel::insert_into(tb_spin_success_process)
            .values(to_vector)
            .execute(&mut CONN.get().unwrap().get().expect("failed connect db"));
    }
}
