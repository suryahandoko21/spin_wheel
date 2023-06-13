use serde::{Serialize, Deserialize};

#[derive(Debug)]
enum VersionData { Version1, Version2 }
#[derive(Serialize,Deserialize)]
pub struct GenericResponse {
    pub status: String,
  
}


impl GenericResponse {
    pub fn new(
        status: String,
      
    )->Self{
        GenericResponse{
            status,
            
        }

    }
}

