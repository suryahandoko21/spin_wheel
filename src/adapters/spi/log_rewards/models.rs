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
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_log_rewards)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LogRewards {
    pub id: i32,
    pub reward_before: String,
    pub reward_after: String,
    pub companies_code: String,
    pub created_at: SystemTime,
    pub created_by: String,
}

#[derive(
    Queryable, Selectable, Insertable, AsChangeset, Debug, Deserialize, QueryableByName, Serialize,
)]
#[diesel(table_name = crate::adapters::spi::cfg::schema::tb_log_rewards)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LogRewardsToDb {
    pub reward_before: String,
    pub reward_after: String,
    pub companies_code: String,
    pub created_at: SystemTime,
    pub created_by: String,
}
