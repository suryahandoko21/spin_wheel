use chrono::NaiveDateTime;
use diesel::prelude::Queryable;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct LogEntity {
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

impl LogEntity {
    pub fn new(
        id: i32,
        companies_code: String,
        before: String,
        after: String,
        change: String,
        remote_ip: String,
        action_change: String,
        entity_type: String,
        created_at: NaiveDateTime,
        created_by: String,
    ) -> Self {
        LogEntity {
            id,
            companies_code,
            before,
            after,
            change,
            remote_ip,
            action_change,
            entity_type,
            created_at,
            created_by,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[allow(non_snake_case)]
pub struct LogCustomEntity {
    pub id: i32,
    pub createdByUser: Option<UserEntity>,
    pub createdDate: String,
    pub lastModifiedDate: String,
    pub entityType: String,
    pub valueBefore: String,
    pub valueAfter: String,
    pub value: String,
    pub user: Option<UserEntity>,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[allow(non_snake_case)]
pub struct UserEntity {
    pub username: String,
}
