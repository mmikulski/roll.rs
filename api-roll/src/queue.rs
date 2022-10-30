use rdkafka::{ClientConfig, producer::FutureProducer};
use uuid::Uuid;
use rdkafka::producer::FutureRecord;
use std::time::Duration;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use derive_more::{Display, From};
use serde::{Deserialize, Serialize};

pub(crate) fn make_producer() -> FutureProducer {
    let mut config = ClientConfig::new();
    std::env::vars()
        .filter(|(key, _)| key.starts_with("KAFKA_"))
        .for_each(|(key, value)| {
            let name = key.trim_start_matches("KAFKA_").to_lowercase();
            config.set(name, value);
        });

    config
        .set("message.timeout.ms", "5000")
        .create::<FutureProducer>()
        .expect("Producer creation error")
}

#[derive(Display, From, Debug)]
pub enum SampleError {
    KafkaError
}

impl std::error::Error for SampleError {}

impl ResponseError for SampleError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            SampleError::KafkaError => HttpResponse::InternalServerError().body("Error sending to kafka"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SampleData {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) roll_for: String,
    pub(crate) result: String
}

pub async fn send_roll_message(
    producer: &FutureProducer,
    sample: &SampleData,
) -> Result<(), SampleError> {
    let msg = json!({
        "messageId" : Uuid::new_v4(),
        "type" : "roll_request",
        "data" : sample
    });

    producer
        .send(
            FutureRecord::to("sample-received")
                .key("messageId")
                .payload(&msg.to_string()),
            Duration::from_secs(0),
        )
        .await
        .map_err(|_| SampleError::KafkaError)
        .map(|_| ())
}
