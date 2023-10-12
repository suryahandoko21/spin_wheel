use actix_http::StatusCode;
use crate::adapters::api::spin_reward::spin_reward_payload::{SpinRewardPayload, SpinRewardUpdatedPayload};
use super::response::{ErrorResponse, GenericResponse};

pub fn filter_zonk_active(payload :&SpinRewardPayload)->(StatusCode,bool,ErrorResponse){
    let mut error_msg = ErrorResponse{
        message: "".to_string(),
        status: "".to_string()
    };
    
    let payload = &payload.detail;
    let target_key = "NONE";
    let target_value = "active";
    // Use filter to create a new iterator with only the matching elements
    let filtered_list: Vec<_> =payload
        .into_iter()
        .filter(|item| item.category == target_key && item.status == target_value)
        .collect();
    if filtered_list.len() < 1{
        error_msg.message = "One Zonk Property must exist and set status is active!!".to_string();
        error_msg.status=  "error".to_string();
        return  (StatusCode::NOT_ACCEPTABLE,true,error_msg);      
    }
    return  (StatusCode::OK,false,error_msg);   
}

pub fn filter_zonk_active_update(payload :&SpinRewardUpdatedPayload)->(StatusCode,bool,ErrorResponse){
    let mut error_msg = ErrorResponse{
        message: "".to_string(),
        status: "".to_string()
    };
    let payload = &payload.detail;
    
    let target_key = "NONE";
    let target_value = "active";
    // Use filter to create a new iterator with only the matching elements
    let filtered_list: Vec<_> =payload
        .into_iter()
        .filter(|item| item.category == target_key && item.status == target_value)
        .collect();
    if filtered_list.len() < 1{
        error_msg.message = "One Zonk Property must exist and set status is active!!".to_string();
        error_msg.status=  "error".to_string();
        return  (StatusCode::NOT_ACCEPTABLE,true,error_msg);    
    }
    return  (StatusCode::OK,false,error_msg); 
}

pub fn reponse_status(result:&GenericResponse)->(StatusCode,bool,ErrorResponse){
    let mut error_msg = ErrorResponse{
        message: "".to_string(),
        status: "".to_string()
    };
    if result.status == "Failed"{
        error_msg.message = result.message.to_string();
        error_msg.status=  "error".to_string();
        return  (StatusCode::NOT_ACCEPTABLE,true,error_msg);  
    }
    return  (StatusCode::OK,false,error_msg);   
}
