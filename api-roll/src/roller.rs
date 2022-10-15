use actix_web::{get, HttpResponse, Responder, web};
use rand::Rng;

#[get("/roll")]
async fn roll() -> impl Responder {
    let mut rng = rand::thread_rng();
    let result: u8 = rng.gen_range(1..10);

    HttpResponse::Ok().body(result.to_string())
}

#[get("/roll/{dice}")]
async fn roll_dice(path: web::Path<String>) -> impl Responder {
    let mut rng = rand::thread_rng();
    let dice = path.into_inner();

    let sides = match dice.as_str() {
        "k2" => Some(2),
        "k4" => Some(4),
        "k6" => Some(6),
        "k8" => Some(8),
        "k10" => Some(10),
        "k12" => Some(12),
        "k20" => Some(20),
        "k100" => Some(100),
        _ => None,
    };

    if let Some(sides) = sides {
        let result: u8 = rng.gen_range(1..=sides.to_owned());
        HttpResponse::Ok().body(result.to_string())
    } else {
        HttpResponse::BadRequest().body("Incorrect dice")
    }
}
