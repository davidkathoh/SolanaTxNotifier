use actix_web::{get,post,web,App,HttpResponse,HttpServer,Responder};
use std::env;
use  dotenv::dotenv;


#[get("/")]
async fn hello() -> impl Responder{
    let api_key = env::var("API_KEY").expect("You haven't set your api key");
    HttpResponse::Ok().body(api_key)
}

#[post("/rpc")]
async fn rpc_webhook(req_body:String) -> impl Responder{
    HttpResponse::Ok().body(req_body)
}
#[post("/telegram")]
async fn telegram_webhook(req_body:String) -> impl Responder{
    HttpResponse::Ok().body(req_body)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
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
