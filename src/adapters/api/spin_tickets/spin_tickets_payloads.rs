use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct SpinTicketPayload {
    // implement for POST/UPDATE requests
    pub userUuId: String,
    pub username: String,
    pub spinTickets :Vec<SpinTickets>,

}
#[derive(Serialize, Deserialize, Debug,Clone)]
pub  struct SpinTickets {
   pub id:i32,
   pub uuid:String,
   pub status:String,
   pub ticketNumber: String,
   pub userId:i32,
   pub userName:String,
   pub pointRuleId:i32,
   pub pointRuleName :String,
   pub expiredType:String,
   pub expiredValue:i32,
   pub ticketCreatedDate:String,
   pub ticketExpiredDate:String
}
