
use diesel::prelude::*;

#[derive(Queryable, Selectable,Insertable,Identifiable)]
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



#[derive(Queryable,Insertable,Debug)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_prizes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinPrizesToDB {
    pub prize_weight: i32,
    pub prize_name : String,
    pub prize_note : String,
    pub prize_category :String,
    pub prize_amount :i32
}