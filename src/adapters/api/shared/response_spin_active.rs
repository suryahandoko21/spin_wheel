use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
#[allow(non_snake_case)]
pub struct ResponseActiveSpin {
   pub enableSpinWheelFeature : bool,
  }
