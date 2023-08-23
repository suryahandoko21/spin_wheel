use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct RequestBeResult {
    pub ticket_uuid : String,
    pub prize :  String
  
}
