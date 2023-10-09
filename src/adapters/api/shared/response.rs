use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::domain::spin_reward_entity::SpinRewardEntity;


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
    pub reward : Option<SpinRewardEntity>,
  }
  
  impl SpinResponse {
    pub fn new(
        status: String,
        reward :Option<SpinRewardEntity>
    )->Self{
        SpinResponse{
            status,
            reward
            
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
pub struct ErrorResponse {
    pub status: String,
    pub message : String,
}

impl ErrorResponse {
    pub fn new(
        status: String,
        message : String)->Self{
            ErrorResponse{
                status,
                message,
            }

        }
    }


