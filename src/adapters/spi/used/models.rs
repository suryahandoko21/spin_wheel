use chrono::NaiveDateTime;
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable,Debug,Deserialize,QueryableByName,Serialize)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_used)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinUseds {
    pub id: i32,
    pub user_id : String, 
    pub created_at : NaiveDateTime, 
    pub updated_at : NaiveDateTime,
    pub created_by : String,
    pub updated_by : String,
    pub used_status : String,
    pub prize_id : i32,
    pub company_id :i32
}