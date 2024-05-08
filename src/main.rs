use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, ResponseError, Result};
use std::{collections::HashMap, env};
use  dotenv::dotenv;
use teloxide::{ prelude::*,};
use serde_json::Value;
use std::sync::Mutex;
use rand::Rng;


#[get("/")]
async fn hello(bot:web::Data<Bot>, account:web::Data<TrackedAddress>) -> impl Responder{

    let user_id = UserId(5331817989);
    let _ = bot.send_message(ChatId::from(user_id),"Hello world").await;

    let mut acc_ = account.account.lock().unwrap();
     if let Some(chat_ids) = acc_.get_mut("0x") {
        println!("John's numbers: {:?}", chat_ids);
        let rnd_id:u64 = rand::thread_rng().gen();
        chat_ids.push(rnd_id);
    }else{
    acc_.insert("0x".to_string(), vec![34,56]);
    }
    
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
    let _ = bot.send_message(chat_id, "gdg").await;
    println!("message sent");
    HttpResponse::Ok()
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
