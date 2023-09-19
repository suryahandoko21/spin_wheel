
use crate::adapters::api::shared::{init_global::GLOBAL_INIT, response_limit::ResponseLimitSpin, request_limit::RequestLimitSpin};

pub async fn get_request_limit(uuid:&mut String)->bool{
    let req_limit = RequestLimitSpin{
        userUUID: uuid.to_string(),
    };
    let global_map = GLOBAL_INIT.get().unwrap();
    let url_spin_limit_be = &global_map["url_spin_limit_be"];
    let token_validation_be = &global_map["token_validation_be"];
    let limit_spin = &global_map["limit_spin"].parse().unwrap_or(false);
    if *limit_spin{
        let client = awc::Client::default();
        let res = client.get(url_spin_limit_be)
            .insert_header(("spinWheelEngineSecretKey", token_validation_be.to_string()))
            .send_json(&req_limit)
            .await;
        let rstatus =  res.as_ref();
        let status = rstatus.ok().unwrap().status();
        if status == 200 {
            let body = res.ok().unwrap().body().await.ok();
            let rs:ResponseLimitSpin = serde_json::from_slice(&body.unwrap()).unwrap();
            println!("response get{:?}",rs);
            if rs.status == "NOT_AVAILABLE"{
              return false;
            }  
            return true;
        }
        return false;
    }
    return true;
}

