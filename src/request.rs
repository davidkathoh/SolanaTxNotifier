use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};



pub   async fn add_address(address:String) {

    let mut helius = get_webhook().await.unwrap();
    helius.accountAddresses.push(address);

    let api_key: String = std::env::var("HELIUS_KEY").expect("HELIUS_KEY must be set.");
    let webhook_id: String = std::env::var("WEBHOOK_ID").expect("webhook_id must be set.");
    let url = format!("https://api.helius.xyz/v0/webhooks/{}?api-key={}", webhook_id, api_key);
    let client  = reqwest::Client::new();
    let response = client
        .put(url)
        .header(CONTENT_TYPE,"application/json")
        .json(&helius)
        .send()
        .await
        .unwrap();
    println!("Success! {:?}",response.text().await)
    }

async fn get_webhook()-> Result<HeliusRequest,String>{
    let api_key: String = std::env::var("HELIUS_KEY").expect("HELIUS_KEY must be set.");
    let webhook_id: String = std::env::var("WEBHOOK_ID").expect("webhook_id must be set.");
    let url = format!("https://api.helius.xyz/v0/webhooks/{}?api-key={}", webhook_id, api_key);
    let response = reqwest::get(
        url).await.unwrap();

    match response.status() {
        reqwest::StatusCode::OK =>{
            Ok(response.json::<HeliusRequest>().await.unwrap())
        }
        other=>{
            Err(format!("Something unexpected happened: {:?}",other))
        }

    }

}
// async fn () -> Result<(), Box<dyn std::error::Error>> {
//     let client = reqwest::Client::builder()
//         .build()?;
//
//     let request = client.request(reqwest::Method::GET, "https://api.helius.xyz/v0/webhooks/266f8752-acea-4877-bdb9-147ac3cd96d7?api-key=678ad639-443a-4f59-92c1-1bea5aef676b");
//
//     let response = request.send().await?;
//     let body = response.text().await?;
//
//     println!("{}", body);
//
//     Ok(())
// }Ok
#[derive(Serialize,Deserialize,Debug)]
struct HeliusRequest {
    webhookURL: String,
    transactionTypes: Vec<String>,
    accountAddresses: Vec<String>,
    webhookType: String,
    authHeader: String,
}