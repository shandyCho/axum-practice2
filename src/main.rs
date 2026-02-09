// main.rs는 어플리케이션의 진입점이며 라우터 설정등을 진행할 수 있다
// main.rs에서 다른 모듈의 요소를 사용하고자 할 때는 해당 모듈에 대해서 mod 키워드를 사용해서 선언해야한다.
// 크레이트 루트 (crate root) 는 러스트 컴파일러가 컴파일을 시작하는 소스 파일이고, 크레이트의 루트 모듈을 구성합니다.
mod dashboard;
mod config;
mod send_view_log;

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::{get};
use axum::{Json, Router, middleware};
use serde::{Serialize};
use tower::ServiceBuilder;

// Kafka 연결 설정을 위한 패키지
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::sync::Arc;

use crate::config::structs::config_structurs::{Getter, KafkaState};
use crate::dashboard::dashboard_handler::{load_all_dashboard, load_some_dashbaord};

// JSON 직렬화를 위한 트레이트를 자동으로 구현
#[derive(Serialize)]
struct Message {
    message: String,
}


async fn custom_middleware(request: Request, next: Next) -> Response {
    // Request 파트에서 할 작업
    let response = next.run(request).await;
    // Response 파트에서 할 작업
    response
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init(); // // 로깅 구독자 초기화 및 시작

    // Kafka 연결 설정
    let producer: FutureProducer = ClientConfig::new()
    .set("bootstrap.servers", "localhost:9092") // Your Kafka broker address
    .set("message.timeout.ms", "5000")
    .create()
    .expect("Producer creation error");

    // Kafka 프로듀서를 Arc로 감싸서 공유 가능하게 만듭니다.
    // Pass shared_producer to your Axum routes via `axum::extract::State`
    let kafka_state = KafkaState {
        producer: Arc::new(producer)
    };


    let service_layer = ServiceBuilder::new()
    .layer(config::logging_config::config2::logging_setup2());


    
    // 서버 IP 및 포트 정의
    let addr = "0.0.0.0:3000";
    // 라우터 정의
    let router = Router::new()
    .route("/", get(|| async {" Hello, World!"}))
    .route("/api/v1/hello", get(hello))
    .route("/api/v1/dashboard", get(load_all_dashboard))
    .route("/api/v1/dashboard/{content_number}", get(load_some_dashbaord))
    .layer(service_layer)
    .layer(middleware::from_fn_with_state(kafka_state.getter(), custom_middleware))
    .with_state(kafka_state);



    // 서버 TCP 포트 리스닝을 통한 서버 구동
    let listner = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listner, router).await.unwrap();


}

async fn hello() -> Json<Message>{
    Json(Message { message: String::from("Hello, Axum") })
}