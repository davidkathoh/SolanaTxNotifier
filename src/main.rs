use actix_web::{get,post,web,App,HttpResponse,HttpServer,Responder, Result};
use core::sync;
use std::env;
use  dotenv::dotenv;
use teloxide::prelude::*;


#[get("/")]
async fn hello() -> impl Responder{


    HttpResponse::Ok().body("Hello world")
}

#[post("/rpc")]
async fn rpc_webhook(req_body:String) -> impl Responder{
    HttpResponse::Ok().body(req_body)
}
#[post("/telegram")]
async fn telegram_webhook(update:teloxide::types::Update,) -> Result<HttpResponse,Box<dyn std::error::Error + Send + sync>>{
    println!("Received update: {:?}",update);
    Ok(HttpResponse::Ok().finish())
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    pretty_env_logger::init();
    let bot = Bot::from_env();

    HttpServer::new(||{
        App::new()
        .service(hello)
        .service(rpc_webhook)
        .service(telegram_webhook)
    })
    .bind(("0.0.0.0",8080))?
    .run()
    .await
   
}
