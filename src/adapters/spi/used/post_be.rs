use crate::adapters::api::{shared::{request_be::RequestBeResult, init_global::GLOBAL_INIT, response_be::{ResponseBeResult, ResponseBeErrorResult}}, slack::push_notif::push_notification};

pub async fn post_to_be(post_be:RequestBeResult,url_address:String)->(bool,String,String,i32){
    let global_map = GLOBAL_INIT.get().unwrap();
    let url_prefix = "services/backend/api/spinwheel/callback/submit-rewards";
    let address = format!("{}/{}", url_address, url_prefix);
    println!("cron{:?}",address);
    let mut bool =false; 
    let mut status = "FAILED".to_string();  
    let mut message = "".to_string();
    let mut status_code = 504;
    let token_validation_be = &global_map["token_validation_be"]; 
    let client = awc::Client::default();
    let response = client.post(address)
    .insert_header(("spinWheelEngineSecretKey", token_validation_be.to_string()))
    .send_json(&post_be).await;
    if !&response.is_err(){
        let mut response_value=  response.ok().unwrap();
        let status_response= &response_value.status();
        let body = &response_value.body().await.ok().unwrap();
        if *status_response == 500{
            let rs:ResponseBeErrorResult = serde_json::from_slice(&body).unwrap();
            return (bool,rs.title,rs.detail,rs.status);
        }
        let rs:ResponseBeResult = serde_json::from_slice(&body).unwrap();
        if rs.statusCode == 200{
           bool = true;
        }
        status = rs.status;
        message = rs.message;
        status_code = rs.statusCode;
    }
    else{
        let send_error_slack =  format!("{}❌{}", url_address, "=> Error BE sending unreachable !!");
        let _x = push_notification(send_error_slack).await;
    }
    return (bool,status,message,status_code);
}

