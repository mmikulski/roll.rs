extern crate env_logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Data;
use actix_web::middleware::Logger;

mod roller;
mod queue;

use crate::roller::{roll, roll_dice};
use crate::queue::{make_producer};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let producer = make_producer();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(Data::new(producer.clone()))
            .service(hello)
            .service(echo)
            .service(roll)
            .service(roll_dice)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
