use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, ResponseError, Result};
use std::env;
use  dotenv::dotenv;
use teloxide::{dispatching::dialogue::GetChatId, prelude::*, types::UpdateKind};


#[get("/")]
async fn hello() -> impl Responder{


    HttpResponse::Ok().body("Hello world")
}

#[post("/rpc")]
async fn rpc_webhook(req_body:String) -> impl Responder{
    HttpResponse::Ok().body(req_body)
}
#[post("/telegram")]
async fn telegram_webhook(body: web::Json<Update>,bot:web::Data<Bot>) -> impl Responder{
    println!("Received update: {:?}",body.id);
    let update = body.0;
    let chat_id = update.chat().unwrap().id;
    println!("Received chat id: {:?}",chat_id.0);
    if let UpdateKind::Message(message) = update.kind {
        let chat__id = message.chat.id;
        println!("Another chat id: {:?}",chat__id.0);
        bot.send_message(chat__id, "gdg").await;
    }
   
 
   
   println!("message sent");
   
   
    
      

    
    HttpResponse::Ok()
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    pretty_env_logger::init();
    let bot = Bot::from_env();

    HttpServer::new(move||{
        App::new()
        .app_data(web::Data::new(bot.clone()))
        .service(hello)
        .service(rpc_webhook)
        .service(telegram_webhook)
    })
    .bind(("0.0.0.0",8080))?
    .run()
    .await
   
}
