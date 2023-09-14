use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
#[allow(non_snake_case)]
pub struct RequestBeResult {
    pub ticketUuid : String,
    pub userId : String,
    pub rewardName :  String,
    pub rewardDescriptions: String,
    pub status :String,
    pub rewardType:String,
    pub money :i32
}


