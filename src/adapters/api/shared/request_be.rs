use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct RequestBeResult {
    pub ticketUuid : String,
    pub userId : String,
    pub rewardName :  String,
    pub status :String,
    pub rewardType:String,
    pub money :i32
   
}
