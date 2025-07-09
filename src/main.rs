// main.rs는 어플리케이션의 진입점이며 라우터 설정등을 진행할 수 있다
// main.rs에서 다른 모듈의 요소를 사용하고자 할 때는 해당 모듈에 대해서 mod 키워드를 사용해서 선언해야한다.
// 크레이트 루트 (crate root) 는 러스트 컴파일러가 컴파일을 시작하는 소스 파일이고, 크레이트의 루트 모듈을 구성합니다.
mod dashboard;

use axum::routing::{get};
use axum::{Json, Router};
use serde::{Serialize};

use crate::dashboard::dashboard_handler::load_dashboard;

// JSON 직렬화를 위한 트레이트를 자동으로 구현
#[derive(Serialize)]
struct Message {
    message: String,
}

#[tokio::main]
async fn main() {
    // 서버 IP 및 포트 정의
    let addr = "0.0.0.0:3000";
    // 라우터 정의
    let router = Router::new()
    .route("/", get(|| async {" Hello, World!"}))
    .route("/api/v1/hello", get(hello))
    .route("/api/v1/dashboard", get(load_dashboard));

    // 서버 TCP 포트 리스닝을 통한 서버 구동
    let listner = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listner, router).await.unwrap();
}

async fn hello() -> Json<Message>{
    Json(Message { message: String::from("Hello, Axum") })
}