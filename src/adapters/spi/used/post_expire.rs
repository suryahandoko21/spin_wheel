use log::{error, info, warn};

use crate::adapters::api::{
    shared::{
        expire_ticket::RequestExpire, init_global::GLOBAL_INIT,
        response_be::ResponseBeTicketExpireResult,
    },
    slack::push_notif::push_notification,
};

pub async fn post_expire(post_be: RequestExpire, url_address: String) -> bool {
    let global_map = GLOBAL_INIT.get().unwrap();
    let url_prefix = "services/backend/api/spinwheel/callback/coupon/expired";
    let address = format!("{}/{}", url_address, url_prefix);
    let token_validation_be = &global_map["token_validation_be"];
    let client = awc::Client::default();
    let mut bool = false;
    let response = client
        .post(address)
        .insert_header(("spinWheelEngineSecretKey", token_validation_be.to_string()))
        .send_json(&post_be)
        .await;

    if !&response.is_err() {
        let mut response_value = response.ok().unwrap();
        let status_response = &response_value.status();
        let body = &response_value.body().await.ok().unwrap();
        if *status_response == 500 {
            error!("Ticket sending Failed error 500 {:?}", &post_be);
        }
        let rs: ResponseBeTicketExpireResult = serde_json::from_slice(&body).unwrap();
        if rs.statusCode == 200 {
            info!("Ticket Expire sending success {:?}", &post_be);
        } else {
            warn!("Ticket Expire sending success NOt Macthing {:?}", &post_be);
        }
        bool = true;
    } else {
        error!("Ticket sending Failed {:?}", &post_be);
        let send_error_slack = format!(
            "{}❌{}",
            url_address, "=> Error BE sending ticket expire unreachable !!"
        );
        let _x = push_notification(send_error_slack).await;
    }
    bool
}
