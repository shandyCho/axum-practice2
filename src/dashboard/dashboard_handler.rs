// JSON 형태로 응답하기 위해서 axum의 Json 구조체를 사용한다.
use axum::{
    extract::{Extension, Path}, http::{
        header, HeaderMap, HeaderValue, StatusCode
    }, response::IntoResponse, Json
};

// mod.rs에 정의한 요소들을 사용하고자 할 때는 use문을 쓰나 가장 먼저 crate 에서 시작해야한다.
use crate::dashboard::structs::{Dashboard, LikeContent};

#[axum::debug_handler] 
pub async fn load_all_dashboard() -> impl IntoResponse {

    let like_list = vec![
        LikeContent::new(
            String::from("TK from 凛として時雨 Whose Blue Tour 2025"),
            String::from("4월 16일 발매한 Whose Blue 앨범 관련 투어 공연입니다."),
            1,
        ),
        LikeContent::new(
            String::from("내일도 출근인가"),
            String::from("제발 내일은 지하철이 고장나길 바라는 직장인의 한숨 섞인 혼잣말"),
            2,
        ),
    ];

    let dashboard = Dashboard::new(
        String::from("shandyCho"), 
        String::from("내 이름은 shandyCho 입니다. shandy는 좋아하는 노래의 제목에서 가져왔습니다."),
        like_list
        );
    let dashboard_string = format!("{:?}", dashboard.clone());

    let body = Json(dashboard.clone());
    
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
    (
        StatusCode::OK,
        headers,
        Extension(dashboard_string),
        body
    )
}

// 스칼라 타입 그대로 받을 것인지 struct 만들어서 받고 Some/None 매치문 태워서 분기처리 할 지 생각해보기
pub async fn load_some_dashbaord(Path(content_number): Path<u32>) -> impl IntoResponse {
    tracing::info!("content number: {:?}", content_number);
    let mut header = HeaderMap::new();
    // header.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
    header.insert(header::CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    match content_number {
        1 => tracing::info!("콘텐츠 넘버는 1입니다"),
        2 => tracing::info!("콘텐츠 넘버는 2입니다"),
        _ => tracing::info!("콘텐츠 넘버는 1도 2도 아닙니다"),
    };
    (
        StatusCode::OK,
        header,
        "우리는 이제 리퀘스트를 받을 수 있어요!".to_string()
    )
}

