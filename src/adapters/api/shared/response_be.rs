use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct ResponseBeResult {
    pub data: Option<Data>,
    pub message: String,
    pub status: String,
    pub statusCode: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Data {
    pub id: i32,
    pub ticketId: i32,
    pub ticketUuid: String,
    pub ticketExpired: String,
    pub userId: i32,
    pub userName: String,
    pub claimStatus: String,
    pub rewardName: String,
    pub rewardType: String,
    pub rewardValue: f64,
    pub walletBefore: f64,
    pub walletAfter: f64,
    pub rewardDescriptions: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct ResponseBeErrorResult {
    pub r#type: String,
    pub title: String,
    pub status: i32,
    pub detail: String,
    pub path: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct ResponseBeTicketExpireResult {
    pub statusCode: i32,
    pub status: String,
    pub message: String,
}

