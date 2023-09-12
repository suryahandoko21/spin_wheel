use std::collections::HashMap;

use serde::{Serialize, Deserialize};


#[derive(Serialize,Deserialize,Debug)]
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
    pub category : String,
    pub reward : String,
    pub description : String
  }
  
  impl SpinResponse {
    pub fn new(
        status: String,
        category : String,
        reward: String,
        description : String
    )->Self{
        SpinResponse{
            status,
            category,
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
    pub spin_available :  i64
  
}

impl SpinAvailableResponse {
    pub fn new(
        status: String,
        message : String,
        spin_available :  i64
      
    )->Self{
        SpinAvailableResponse{
            status,
            message,
            spin_available
            
        }

    }
}

#[derive(Serialize,Deserialize)]
pub struct JwtResponse {
    pub status: String,
    pub message : String,
}

impl JwtResponse {
    pub fn new(
        status: String,
        message : String)->Self{
            JwtResponse{
                status,
                message,
            }

        }
    }


