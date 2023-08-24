// use crate::adapters::api::shared::request_be::RequestBeResult;
pub async fn fetch_company_list()->bool{
    // let mut bool =true;    
    let client = awc::Client::default();
    let res = client.get("https://query.lidoapi.com/companies/all")
        // .insert_header(("User-Agent", "my-app/1.2"))
        .send()
        .await;
    println!("result{:?}",res);
    return true;
    
}
