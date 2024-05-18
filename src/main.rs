mod request;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, ResponseError, Result};
use std::{collections::HashMap, default};
use std::env::args;
use  dotenv::dotenv;
use teloxide::{ prelude::*, types::UpdateKind};
use serde_json::Value;
use std::sync::Mutex;
use once_cell::unsync::Lazy;
use request::add_address;





#[get("/")]
async fn hello(bot:web::Data<Bot>, _account:web::Data<TrackedAddress>) -> impl Responder{

    let user_id = UserId(5331817989);
    let _ = bot.send_message(ChatId::from(user_id),"Hello world").await;
    add_address("9Jt8mC9HXvh2g5s3PbTsNU71RS9MXUbhEMEmLTixYirb".to_string()).await;
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
async fn telegram_webhook(body: web::Json<Update>,bot:web::Data<Bot>,account:web::Data<TrackedAddress>) -> impl Responder{
    let update = body.0;
    let chat_id = update.chat().unwrap().id;
    println!("Received chat id: {:?}",chat_id.0);
   
    
    match update.kind {
        UpdateKind::Message(message)=>{
            if let Some(text) = message.text() {
                // Process the text message
                println!("Received text: {}", text);

                let _ = bot.send_message(chat_id, text).await;
                let mut acc_ = account.account.lock().unwrap();
            if let Some((command, args)) = teloxide::utils::command::parse_command_with_prefix("/",&text,"") {
                match command {
                   
                    "address" => {
                        let response =  args.join("");
                        if args.len()>1 {
                            let address = args[1];
                            bot.send_message(chat_id, format!("Address provided {}",address)).await;
                        }else {
                            bot.send_message(chat_id, "No address in the comand").await;
                        }
                 if let Some(telegram_ids) = acc_.get_mut(&response) {
                    telegram_ids.push(chat_id.0);
                    // add_address(response.clone()).await;

                   let _ = bot.send_message(chat_id, response).await;
                }else{
                acc_.insert(response, vec![chat_id.0]);
                }
            }
                   
                    "list" => {
                        
                        
                            let key_array:Vec<String> = acc_.clone().into_keys().collect();
                            let _ = bot.send_message(chat_id,key_array.join("-")).await;
                        
                       
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
    account:Mutex<HashMap<String,Vec<i64>>>,
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
