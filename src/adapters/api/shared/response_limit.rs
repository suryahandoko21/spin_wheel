use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
#[allow(non_snake_case)]
pub struct ResponseLimitSpin {
   pub userId : String,
   pub userUUID:String,
   pub userName:String,
   pub spinCount:i32,
   pub maximumSpinPerDay:i32,
   pub status: String,
  }
