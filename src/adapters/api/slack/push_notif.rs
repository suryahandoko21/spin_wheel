use serde_json::json;
pub async fn push_notification(_notif:String){
    /* Masih belum selesai */
    let data = json!({
        
            "payload": "{\"channel\": \"#my-channel-here\", \"username\": \"webhookbot\", \"text\": \"This is posted to #my-channel-here and comes from a bot named webhookbot.\", \"icon_emoji\": \":ghost:\"}"
        
    });
    let _json_string = serde_json::to_string(&data).unwrap();
    let client = awc::Client::default();

    let _res = client.post("https://hooks.slack.com/services/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")
    .insert_header(("Content-Type", "application/x-www-form-urlencoded"))
    .send_json(&data)
    .await;
} 

