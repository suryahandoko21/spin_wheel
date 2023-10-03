use crate::adapters::api::shared::{request_be::RequestBeResult, init_global::GLOBAL_INIT};

pub async fn post_to_be(post_be:RequestBeResult,url_address:String)->bool{
    let global_map = GLOBAL_INIT.get().unwrap();
    let url_prefix = "services/backend/api/spinwheel/callback/submit-rewards";
    let address = format!("{}/{}", url_address, url_prefix);
    let mut bool =true;   
    let token_validation_be = &global_map["token_validation_be"]; 
    let client = awc::Client::default();
    let response = client.post(address)
    .insert_header(("spinWheelEngineSecretKey", token_validation_be.to_string()))
    .send_json(&post_be).await;
    if !response.is_err(){
        let status = response.ok().unwrap().status();
        if status == 200 {
            bool = true;
        }
    }
    return bool;
}

