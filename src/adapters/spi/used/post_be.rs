use crate::adapters::api::shared::request_be::RequestBeResult;
pub async fn post_to_be(webhook_be:String,post_be:RequestBeResult)->bool{
    let mut bool =true;    
    let client = awc::Client::default();
    let response = client.post(webhook_be)
    .send_json(&post_be).await;
    if response.is_err(){ 
        bool = false;
    }
    return bool;
    
}
