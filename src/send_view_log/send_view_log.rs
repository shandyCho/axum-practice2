use std::sync::Arc;

use axum::{extract::State, http::StatusCode};
use rdkafka::producer::{FutureProducer, FutureRecord};




pub async fn send_view_log(State(producer): State<Arc<FutureProducer>>, content_classification: String, user_id: u32) {
    // 여기에 카프카로 유저가 본 게시글의 주제와 유저 id 값을 메세지로 넘긴다.
    tracing::info!("send view log to kafka");

    let record = FutureRecord::to("my_topic") // Your Kafka topic
        .payload(&content_classification)    // message payload
        .key("some_key"); // Optional key

    match producer.send(record, tokio::time::Duration::from_secs(0)).await {
        Ok((_partition, _offset)) => (StatusCode::OK, "Message sent".to_string()),
        Err((e, _message)) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to send message: {:?}", e)),
    };
    
    tracing::info!("{}", "아아아아")
}
