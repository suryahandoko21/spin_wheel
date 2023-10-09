use std::time::SystemTime;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
#[derive(Queryable, Selectable,Insertable,Identifiable,AsChangeset,Debug,Deserialize,QueryableByName,Serialize)]
// #[derive(Queryable, Selectable,Debug,Deserialize,QueryableByName,Serialize,Clone,Insertable)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_failed_process)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FailedProcess {
    pub id: i32,
    pub ticket_uuid :String,
    pub user_id : String, 
    pub reward_name : String,
    pub status :String,
    pub reward_type :String,
    pub reward_description:String,
    pub money :i32,
    pub post_status : String,
    pub failed_message :String,
    pub url_address :String,
    pub created_at : SystemTime, 
    pub updated_at : SystemTime, 
  }

#[derive(Queryable, Selectable,Debug,Deserialize,QueryableByName,Serialize,Clone,Insertable)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_failed_process)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FailedProcessToDb {
    pub ticket_uuid :String,
    pub user_id : String, 
    pub reward_name : String,
    pub status :String,
    pub reward_type :String,
    pub reward_description:String,
    pub money :i32,
    pub post_status : String,
    pub failed_message :String,
    pub url_address :String,
    pub created_at : SystemTime, 
    pub updated_at : SystemTime, 
  }
