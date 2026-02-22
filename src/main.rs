// main.rs는 어플리케이션의 진입점이며 라우터 설정등을 진행할 수 있다
// main.rs에서 다른 모듈의 요소를 사용하고자 할 때는 해당 모듈에 대해서 mod 키워드를 사용해서 선언해야한다.
// 크레이트 루트 (crate root) 는 러스트 컴파일러가 컴파일을 시작하는 소스 파일이고, 크레이트의 루트 모듈을 구성합니다.
mod dashboard;
mod config;
mod auth;

use axum::http::Method;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::routing::{get, post};
use axum::{Json, Router, middleware};
use serde::{Serialize};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::config::application_config::{get_application_config, Config, JwtConfig};
use crate::dashboard::dashboard_handler::load_dashboard;
use crate::auth::{login::login, jwt::verify_jwt};

// JSON 직렬화를 위한 트레이트를 자동으로 구현
#[derive(Serialize)]
struct Message {
    message: String,
}

#[derive(Clone, Debug)]
pub struct JWTConfigState {
    jwt_config: JwtConfig
}

#[tokio::main]
async fn main() {
    let application_config: Config = get_application_config().await;
    let jwt_config_state = JWTConfigState { 
        jwt_config: application_config.get_jwt_config() 
    };
    let axum_config = application_config.get_axum_config(); 
    
    tracing_subscriber::fmt::init(); // 로깅 구독자 초기화 및 시작
    let logging_layer = ServiceBuilder::new()
        // .layer(config::logging_config::config2::logging_setup2())
        .layer(middleware::from_fn_with_state(jwt_config_state.clone(), verify_jwt));

    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .expose_headers([AUTHORIZATION]);   // 프론트엔드에서 Authorization 헤더값에 접근하기 위해 필요함 (Access-Control-Expose-Headers 헤더에 값을 추가해주는 것)

    let global_layer = ServiceBuilder::new()
        .layer(config::logging_config::config2::logging_setup2())
        .layer(cors_layer);
    // 서버 IP 및 포트 정의
    // let addr = "0.0.0.0:3500";
    let addr = axum_config.get_addr();
    // 라우터 정의
    // State를 사용해야 하는 라우터이기 때문에 따로 분리해줌
    let login_router = || -> Router {
        Router::new()
            .route("/login", post(login))
            .with_state(jwt_config_state)
    };

    let router = Router::new()
    .route("/", get(|| async {" Hello, World!"}))
    .route("/api/v1/hello", get(hello))
    .route("/api/v1/dashboard", get(load_dashboard))
    // .route("/api/v1/login", post(login))
    // .route_layer()
    .layer(logging_layer)
    .nest("/api/v1", login_router())
    .layer(global_layer);



    // 서버 TCP 포트 리스닝을 통한 서버 구동
    let listner = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listner, router).await.unwrap();
}

async fn hello() -> Json<Message>{
    Json(Message { message: String::from("Hello, Axum") })
}