use actix_web::{get, HttpResponse, Responder};
use rand::Rng;

#[get("/roll")]
async fn roll() -> impl Responder {
    let mut rng = rand::thread_rng();
    let result: u8 = rng.gen_range(1..10);

//    let throw_result = 12;
    HttpResponse::Ok().body(result.to_string())
}
