use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct RequestExpire {
    pub uuid: String,
    pub status: String,
    pub ipAddress: String,
}
