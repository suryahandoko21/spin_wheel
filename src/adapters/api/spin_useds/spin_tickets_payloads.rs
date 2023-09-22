use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Serialize, Deserialize, Debug,Clone,ToSchema)]
pub struct SpinUsedPayload {
    pub user_uuid: String,
    pub company_code :String
}
