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






