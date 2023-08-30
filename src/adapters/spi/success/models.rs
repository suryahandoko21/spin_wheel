use std::time::SystemTime;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable,Debug,Deserialize,QueryableByName,Serialize,Insertable)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_success_process)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProcessSuccess {
    pub id: i32,
    pub ticket_uuid :String,
    pub user_id : String, 
    pub reward_name : String,
    pub status :String,
    pub reward_type :String,
    pub money :i32,
    pub post_status : String,
    pub created_at : SystemTime, 
  }
  #[derive(Queryable, Selectable,Debug,Deserialize,QueryableByName,Serialize,Clone,Insertable)]
  #[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_success_process)]
  #[diesel(check_for_backend(diesel::pg::Pg))]
  pub struct ProcessSuccessToDb {
      pub ticket_uuid :String,
      pub user_id : String, 
      pub reward_name : String,
      pub status :String,
      pub reward_type :String,
      pub money :i32,
      pub post_status : String,
      pub created_at : SystemTime, 
    }
  