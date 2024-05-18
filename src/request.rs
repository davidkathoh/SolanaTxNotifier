



pub   async fn add_address() {
    let api_key: String = std::env::var("HELIUS_KEY").expect("HELIUS_KEY must be set.");
    let webhook_id: String = std::env::var("webhook_id").expect("webhook_id must be set.");
        let url = format!("https://api.helius.xyz/v0/webhooks/{}?api-key={}", webhook_id, api_key);
        let response = reqwest::get(
        url).await;

        // let  response = request.send().await;
        //let  body= response.json().await?;

        println!("{:?}", response);
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
