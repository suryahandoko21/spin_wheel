use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct ResponseActiveSpin {
    pub enableSpinWheelFeature: bool,
    pub spinWheelLogo: String,
}
