use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct SpinTicketPayload {
    // implement for POST/UPDATE requests
    pub userUUID: String,
    pub ruleid: i32,
    pub userid: i32,
    pub username: String,
    pub spinTickets :Vec<SpinTickets>,

}
#[derive(Serialize, Deserialize, Debug,Clone)]
pub  struct SpinTickets {
   pub  id:i32,
   pub uuid:String,
   pub status:String,
   pub userId:i32,
   pub pointRuleId:i32
}
