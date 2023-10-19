
use std::time::SystemTime;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable,Insertable,Identifiable,AsChangeset,Debug,Deserialize,QueryableByName,Serialize)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_companies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Companies {
    pub id: i32, 
    pub uuid: String,
    pub companies_code: String,
    pub companies_name : String,
    pub companies_address :String,
    pub is_company_enabled :bool,
    pub max_credit:i32,
    pub created_at : SystemTime, 
    pub updated_at : SystemTime,
    pub created_by : String,
    pub updated_by : String,

}
#[derive(Queryable, Selectable,Insertable,AsChangeset,Debug,Deserialize,QueryableByName,Serialize)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_companies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SpinCompaniesToDb {
    pub uuid: String,
    pub companies_code: String,
    pub companies_name : String,
    pub companies_address :String,
    pub is_company_enabled :bool,
    pub created_at : SystemTime, 
    pub updated_at : SystemTime,
    pub created_by : String,
    pub updated_by : String,
}

