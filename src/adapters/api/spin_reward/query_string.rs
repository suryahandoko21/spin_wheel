use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug)]
pub struct QstringReward {
  pub status: Option<String>,
  pub types: Option<String>,
  pub name :Option<String>
}

#[derive(Deserialize,Serialize,Debug)]
pub struct QstringCompany {
  pub company_code: Option<String>
 
}