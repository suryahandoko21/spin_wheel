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
    pub id: Option<i32>,
    pub ticketId: Option<i32>,
    pub ticketUuid: Option<String>,
    pub ticketExpired: Option<String>,
    pub userId: Option<i32>,
    pub userName: Option<String>,
    pub claimStatus: Option<String>,
    pub rewardName: Option<String>,
    pub rewardType: Option<String>,
    pub rewardValue: Option<f64>,
    pub walletBefore: Option<f64>,
    pub walletAfter: Option<f64>,
    pub rewardDescriptions: Option<String>,
    
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
