use crate::adapters::api::shared::{request_be::RequestBeResult, init_global::GLOBAL_INIT};
pub async fn post_to_be(post_be:RequestBeResult)->bool{
    let global_map = GLOBAL_INIT.get().unwrap();
    let webhook_be = &global_map["webhook_be"];
    let mut bool =true;    
    let client = awc::Client::default();
    let response = client.post(webhook_be)
    .send_json(&post_be).await;
    if response.is_err(){ 
        bool = false;
    }
    return bool;
}

