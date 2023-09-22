pub async fn push_notification(){
    let client = awc::Client::default();
    let _res = client.post("https://hooks.slack.com/services/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")
    .insert_header(("Content-Type", "application/x-www-form-urlencoded"))
    .send_body("payload=%7B%22channel%22%3A+%22%23spin_wheel_notif%22%2C+%22username%22%3A+%22webhookbot%22%2C+%22text%22%3A+%22This+is+posted+to+%23my-channel-here+and+comes+from+a+bot+named+webhookbot.%22%2C+%22icon_emoji%22%3A+%22%3Aghost%3A%22%7D")
    .await;
} 