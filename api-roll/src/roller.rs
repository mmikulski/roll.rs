use actix_web::{get, HttpResponse, Responder, /*ResponseError,*/ web};
use rand::Rng;
use rdkafka::producer::{FutureProducer/*, FutureRecord*/};
use uuid::Uuid;

use crate::queue;
use crate::queue::{SampleData, SampleError};

#[get("/roll")]
pub(crate) async fn roll() -> impl Responder {
    let mut rng = rand::thread_rng();
    let result: u8 = rng.gen_range(1..10);

    HttpResponse::Ok().body(result.to_string())
}

#[get("/roll/{dice}")]
pub(crate) async fn roll_dice(path: web::Path<String>, producer: web::Data<FutureProducer>) -> Result<HttpResponse, SampleError> {
    let mut rng = rand::thread_rng();
    let dice = path.into_inner();

    let sides_input = match dice.as_str() {
        "d2" => Some(2),
        "d4" => Some(4),
        "d6" => Some(6),
        "d8" => Some(8),
        "d10" => Some(10),
        "d12" => Some(12),
        "d20" => Some(20),
        "d100" => Some(100),
        _ => None,
    };

    if let Some(sides) = sides_input {
        let result: u8 = rng.gen_range(1..=sides.to_owned());
        let sample = SampleData {
            id: Uuid::new_v4(),
            name: String::from("roll_request_received"),
            roll_for: dice,
            result: result.to_string()
        };

        queue::send_roll_message(&producer, &sample).await?;

        Ok(HttpResponse::Ok().body(result.to_string()))
    } else {
        Ok(HttpResponse::BadRequest().body("Incorrect dice"))
    }


}
