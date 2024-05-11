use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, ResponseError, Result};
use std::{collections::HashMap, default};
use  dotenv::dotenv;
use teloxide::{ prelude::*, types::UpdateKind};
use serde_json::Value;
use std::sync::Mutex;


#[get("/")]
async fn hello(bot:web::Data<Bot>, _account:web::Data<TrackedAddress>) -> impl Responder{

    let user_id = UserId(5331817989);
    let _ = bot.send_message(ChatId::from(user_id),"Hello world").await;

    // let mut acc_ = account.account.lock().unwrap();
    //  if let Some(chat_ids) = acc_.get_mut("0x") {
        
    //     println!("John's numbers: {:?}", chat_ids);
    //     let _ = bot.send_message(ChatId::from(user_id),chat_ids.iter().map(|&id| id.to_string()).collect::<Vec<String>>().join(",")).await;

    //     let rnd_id:u64 = rand::thread_rng().gen();
    //     chat_ids.push(rnd_id);
    // }else{
    // acc_.insert("0x".to_string(), vec![34,56]);
    // }
    
    HttpResponse::Ok().body("Hello world")
}

#[post("/rpc")]
async fn rpc_webhook(req_body:web::Json<Value>,bot:web::Data<Bot>) -> impl Responder{
    let json_data = &req_body;

    
    println!("JSON data as string: {}", json_data.to_string());

    let _ = bot.send_message(ChatId::from(UserId(5331817989)), json_data.to_string()).await;
    if let Some(description) = json_data[0]["description"].as_str() {
        println!("Description: {}", description);
        let _ = bot.send_message(ChatId::from(UserId(5331817989)), description).await;
    } else {
        println!("Description not found or not a string");
    }

    
    HttpResponse::Ok().body("ok")
}
#[post("/telegram")]
async fn telegram_webhook(body: web::Json<Update>,bot:web::Data<Bot>) -> impl Responder{
    let update = body.0;
    let chat_id = update.chat().unwrap().id;
    println!("Received chat id: {:?}",chat_id.0);
   
    
    match update.kind {
        UpdateKind::Message(message)=>{
            if let Some(text) = message.text() {
                // Process the text message
                println!("Received text: {}", text);

                let _ = bot.send_message(chat_id, text).await;

            if let Some((command, args)) = teloxide::utils::command::parse_command_with_prefix("/",&text,"") {
                match command {
                   
                    "address" => {
                        let response = format!("Echo: {}", args.join(""));
                        let _ = bot.send_message(chat_id, response).await;}
                   
                    "list" => {
                        let response = "list of addresses";
                        let _ = bot.send_message(chat_id,response).await;
                    }
                    _ => {}
                }
            }
        
            } else {
                println!("Received message without text.");
            }
            
        }
        _ => println!("Received update other than a message."),
    }

   
    HttpResponse::Ok().body("")
}

fn extract_text_from_command(command: &str) -> Option<&str> {
    if let Some(text_index) = command.find(" ") {
        let text = &command[(text_index + 1)..];
        Some(text)
    } else {
        None
    }
}
struct TrackedAddress{
    account:Mutex<HashMap<String,Vec<u64>>>,
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    pretty_env_logger::init();
    let bot = Bot::from_env();
    let account = web::Data::new(TrackedAddress{account:Mutex::new(HashMap::new())});


    HttpServer::new(move||{
        App::new()
        .app_data(web::Data::new(bot.clone()))
        .app_data(account.clone())
        .service(hello)
        .service(rpc_webhook)
        .service(telegram_webhook)
    })
    .bind(("0.0.0.0",8080))?
    .run()
    .await
   
}
