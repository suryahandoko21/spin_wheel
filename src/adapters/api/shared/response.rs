use std::collections::HashMap;

use serde::{Serialize, Deserialize};


#[derive(Serialize,Deserialize)]
pub struct GenericResponse {
    pub status: String,
    pub message : String
  
}
impl GenericResponse {
    pub fn new(
        status: String,
        message : String
      
    )->Self{
        GenericResponse{
            status,
            message
            
        }

    }
}


#[derive(Serialize,Deserialize)]
pub struct SpinResponse {
    pub status: String,
    pub message : String,
    pub reward : String,
    pub description : String
  }
  
  impl SpinResponse {
    pub fn new(
        status: String,
        message : String,
        reward: String,
        description : String
    )->Self{
        SpinResponse{
            status,
            message,
            reward,
            description,
            
        }

    }
}  




#[derive(Serialize,Deserialize,Debug)]
pub struct TicketResponse {
    pub status: String,
    pub message : String,
    pub data :  Vec<HashMap<String,String>>
  
}


impl TicketResponse {
    pub fn new(
        status: String,
        message : String,
        data :  Vec<HashMap<String,String>>
      
    )->Self{
        TicketResponse{
            status,
            message,
            data
            
        }

    }
}



#[derive(Serialize,Deserialize,Debug)]
pub struct SpinAvailableResponse {
    pub status: String,
    pub message : String,
    pub spin :  i64
  
}


impl SpinAvailableResponse {
    pub fn new(
        status: String,
        message : String,
        spin :  i64
      
    )->Self{
        SpinAvailableResponse{
            status,
            message,
            spin
            
        }

    }
}





