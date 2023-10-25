use std::time::SystemTime;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Debug, Deserialize, QueryableByName, Serialize)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_used)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinUseds {
    pub id: i32,
    pub user_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
    pub updated_by: String,
    pub used_status: String,
    pub prize_id: i32,
    pub companies_code: String,
    pub ticket_uuid: String,
    pub remote_ip :String
}

#[derive(Queryable, Selectable, Debug, Deserialize, QueryableByName, Serialize, Insertable)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_used)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinUsedsToDb {
    pub user_id: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub created_by: String,
    pub updated_by: String,
    pub used_status: String,
    pub prize_id: i32,
    pub companies_code: String,
    pub ticket_uuid: String,
    pub remote_ip :String
}
