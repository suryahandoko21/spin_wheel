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
    pub message : String,
    pub spin_amount :  i64,
    pub available : bool
  
}

impl SpinAvailableResponse {
    pub fn new(
        message : String,
        spin_amount :  i64,
        available : bool
      
    )->Self{
        SpinAvailableResponse{
            message,
            spin_amount,
            available       
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


