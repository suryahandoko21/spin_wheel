use diesel::{prelude::*};
use chrono::NaiveDateTime;
#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_promos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinPromos {
    pub id: i32,
    pub promo_amount : i32,
    pub promo_status : String,
    pub user_id : String,
    pub username :String,
    pub expired_at :NaiveDateTime,
    pub point_currention_time : NaiveDateTime,
    pub created_at : NaiveDateTime, 
    pub updated_at : NaiveDateTime,
    pub created_by : String,
    pub updated_by : String,

}

#[derive(Queryable, Selectable, Debug, PartialEq,Insertable,AsChangeset)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_promos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinPromosToDb {
    pub promo_amount : i32,
    pub promo_status : String,
    pub user_id : String,
    pub username :String,
    pub expired_at :chrono::NaiveDateTime,
    pub point_currention_time : chrono::NaiveDateTime,
    pub created_at : chrono::NaiveDateTime, 
    pub updated_at : chrono::NaiveDateTime,
    pub created_by : String,
    pub updated_by : String,

}
