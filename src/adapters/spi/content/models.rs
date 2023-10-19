use std::time::SystemTime;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable,
    Selectable,
    Insertable,
    Identifiable,
    AsChangeset,
    Debug,
    Deserialize,
    QueryableByName,
    Serialize,
)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_content)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Content {
    pub id: i32,
    pub companies_code: String,
    pub content_title: String,
    pub content_description: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub created_by: String,
    pub updated_by: String,
    pub default: bool,
}

#[derive(
    Queryable, Selectable, Insertable, AsChangeset, Debug, Deserialize, QueryableByName, Serialize,
)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_content)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ContentToDb {
    pub companies_code: String,
    pub content_title: String,
    pub content_description: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub created_by: String,
    pub updated_by: String,
    pub default: bool,
}

#[derive(
    Queryable, Selectable, Insertable, AsChangeset, Debug, Deserialize, QueryableByName, Serialize,
)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_content)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ContentUpdateDb {
    pub content_title: String,
    pub content_description: String,
    pub updated_at: SystemTime,
}
