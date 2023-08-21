use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct SpinListPayload {
    pub company_code: String,
    pub list_status: String,
    pub rule :Vec<SpinListRule>

}


#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct SpinPostPayload {
    pub amount: i32,
    pub uuid : String
   
}
#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct SpinListRule{
    pub ruleid : i32,
    pub spin_prizes_id: i32,
    pub percentage: i32,
}


