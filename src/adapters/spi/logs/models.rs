use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
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
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LogRewards {
    pub id: i32,
    pub companies_code: String,
    pub before: String,
    pub after: String,
    pub change: String,
    pub remote_ip: String,
    pub action_change: String,
    pub entity_type: String,
    pub created_at: NaiveDateTime,
    pub created_by: String,
}

#[derive(
    Queryable, Selectable, Insertable, AsChangeset, Debug, Deserialize, QueryableByName, Serialize,
)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_spin_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LogsToDb {
    pub companies_code: String,
    pub before: String,
    pub after: String,
    pub change: String,
    pub remote_ip: String,
    pub action_change: String,
    pub entity_type: String,
    pub created_at: SystemTime,
    pub created_by: String,
}
