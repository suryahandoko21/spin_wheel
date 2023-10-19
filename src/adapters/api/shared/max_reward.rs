use super::{init_global::GLOBAL_INIT, response::ErrorResponse};
use crate::adapters::api::spin_reward::spin_reward_payload::{
    SpinRewardPayload, SpinRewardUpdatedPayload,
};
use actix_http::StatusCode;

pub fn max_reward_active_add(payload: &SpinRewardPayload) -> (StatusCode, bool, ErrorResponse) {
    let mut error_msg = ErrorResponse {
        message: "".to_string(),
        status: "".to_string(),
    };

    let payload = &payload.detail;
    let target_value = "active";
    // Use filter to create a new iterator with only the matching elements
    let filtered_list: Vec<_> = payload
        .into_iter()
        .filter(|item| item.status == target_value)
        .collect();

    let global_init = GLOBAL_INIT.get().unwrap();
    let max_list_active_reward = &global_init["max_list_active_reward"].parse().unwrap_or(12);
    if filtered_list.len() > *max_list_active_reward as usize {
        error_msg.message = format!(
            "{} {}",
            "Max reward active cant more than".to_string(),
            max_list_active_reward
        );
        error_msg.status = "error".to_string();
        return (StatusCode::NOT_ACCEPTABLE, true, error_msg);
    }
    return (StatusCode::OK, false, error_msg);
}

pub fn max_reward_active_update(
    payload: &SpinRewardUpdatedPayload,
) -> (StatusCode, bool, ErrorResponse) {
    let mut error_msg = ErrorResponse {
        message: "".to_string(),
        status: "".to_string(),
    };

    let payload = &payload.detail;
    let target_value = "active";
    // Use filter to create a new iterator with only the matching elements
    let filtered_list: Vec<_> = payload
        .into_iter()
        .filter(|item| item.status == target_value)
        .collect();

    let global_init = GLOBAL_INIT.get().unwrap();
    let max_list_active_reward = &global_init["max_list_active_reward"].parse().unwrap_or(12);
    if filtered_list.len() > *max_list_active_reward as usize {
        error_msg.message = format!(
            "{} {}",
            "Max reward active cant more than".to_string(),
            max_list_active_reward
        );
        error_msg.status = "error".to_string();
        return (StatusCode::NOT_ACCEPTABLE, true, error_msg);
    }
    return (StatusCode::OK, false, error_msg);
}
