
use std::time::SystemTime;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Queryable, Selectable,Insertable,Identifiable,AsChangeset,Debug,Deserialize,QueryableByName,Serialize)]

#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_rewards)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinRewards {
    pub id: i32, 
    pub reward_name : String,
    pub reward_note : String,
    pub reward_category : String,
    pub reward_amount : i32,
    pub reward_money : i32,
    pub reward_status : String,
    pub reward_order : i32,
    pub companies_code:String,
    pub percentage:f64,
    pub reward_image: String,
    pub created_at : NaiveDateTime, 
    pub updated_at : NaiveDateTime,
}

#[derive(Queryable,Insertable,Debug,AsChangeset)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_rewards)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinRewardToDB {
    pub reward_name : String,
    pub reward_note : String,
    pub reward_category : String,
    pub reward_amount : i32,
    pub reward_money : i32,
    pub reward_status : String,
    pub reward_order : i32,
    pub companies_code:String,
    pub percentage:f64,
    pub reward_image: String,
    pub created_at : SystemTime, 
    pub updated_at : SystemTime,
}

#[derive(Queryable,Insertable,Debug,AsChangeset)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_rewards)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinRewardUpdateToDB {
    pub reward_name : String,
    pub reward_note : String,
    pub reward_category : String,
    pub reward_amount : i32,
    pub reward_money : i32,
    pub reward_status : String,
    pub companies_code:String,
    pub reward_order : i32,
    pub percentage:f64,
    pub reward_image: String,
    pub updated_at : SystemTime,
}
