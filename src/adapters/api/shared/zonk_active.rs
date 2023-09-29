use crate::adapters::api::spin_reward::spin_reward_payload::{SpinRewardPayload, SpinRewardUpdatedPayload};

pub fn filter_zonk_active(payload :&SpinRewardPayload)->bool{
    let payload = &payload.detail;
    let target_key = "zonk";
    let target_value = "active";
    // Use filter to create a new iterator with only the matching elements
    let filtered_list: Vec<_> =payload
        .into_iter()
        .filter(|item| item.category == target_key && item.status == target_value)
        .collect();
    if filtered_list.len() > 0{
        return true;
    }
return false;
}

pub fn filter_zonk_active_update(payload :&SpinRewardUpdatedPayload)->bool{
    let payload = &payload.detail;
    let target_key = "zonk";
    let target_value = "active";
    // Use filter to create a new iterator with only the matching elements
    let filtered_list: Vec<_> =payload
        .into_iter()
        .filter(|item| item.category == target_key && item.status == target_value)
        .collect();
    if filtered_list.len() > 0{
        return true;
    }
return false;
}