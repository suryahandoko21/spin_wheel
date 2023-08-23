
use chrono::format::Numeric;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::adapters::spi::companies::models::Companies; 
#[derive(Queryable, Selectable,Insertable,Identifiable,AsChangeset,Debug,Deserialize,QueryableByName,Serialize,Associations)]
#[diesel(belongs_to(Companies))]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_prizes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinPrizes {
    pub id: i32, 
    pub prize_name : String,
    pub prize_note : String,
    pub prize_category : String,
    pub prize_amount : i32,
    pub prize_money : i32,
    pub companies_id:i32,
    pub percentage:i32
}


#[derive(Queryable,Insertable,Debug,AsChangeset)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_prizes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinPrizesToDB {
    pub prize_name : String,
    pub prize_note : String,
    pub prize_category :String,
    pub prize_amount :i32,
    pub prize_money : i32,
    pub percentage:i32,
    pub companies_id:i32
}


#[derive(Debug, QueryableByName)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_prizes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinPrizesCompanies{
    #[diesel(embed)]
    pub companies :Companies,
    pub prize_name : String,
    pub prize_note : String,
    pub prize_category : String,
    pub prize_amount : i32,
    pub prize_money : i32,
    pub percentage:i32
   
}
