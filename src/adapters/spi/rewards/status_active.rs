use crate::adapters::api::shared::{
    init_global::GLOBAL_INIT, response_spin_active::ResponseActiveSpin,
};

pub async fn status_active_spinwheel(url_address: String) -> bool {
    let global_map = GLOBAL_INIT.get().unwrap();
    let url_prefix = "services/backend/api/spinwheel/callback/get-spinwhere-feature-enable";
    let address = format!("{}/{}", url_address, url_prefix);
    let mut bool = false;
    let token_validation_be = &global_map["token_validation_be"];
    let client = awc::Client::default();
    let response = client
        .get(address)
        .insert_header(("spinWheelEngineSecretKey", token_validation_be.to_string()))
        .send()
        .await;
    let rstatus = response.as_ref();
    if rstatus.is_ok() {
        let status = rstatus.ok().unwrap().status();
        if status == 200 {
            let body = response.ok().unwrap().body().await.ok();
            let rs: ResponseActiveSpin = serde_json::from_slice(&body.unwrap()).unwrap();
            bool = rs.enableSpinWheelFeature;
        }
    }
    return bool;
}
