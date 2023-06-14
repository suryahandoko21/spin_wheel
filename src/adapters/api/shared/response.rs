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

