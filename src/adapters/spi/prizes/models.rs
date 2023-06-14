
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Queryable, Selectable,Insertable,Identifiable,AsChangeset,Debug,Deserialize)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_prizes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinPrizes {
    pub id: i32, 
    pub prize_weight: i32,
    pub prize_name : String,
    pub prize_note : String,
    pub prize_category :String,
    pub prize_amount :i32
}



#[derive(Queryable,Insertable,Debug,AsChangeset)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_prizes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinPrizesToDB {
    pub prize_weight: i32,
    pub prize_name : String,
    pub prize_note : String,
    pub prize_category :String,
    pub prize_amount :i32
}