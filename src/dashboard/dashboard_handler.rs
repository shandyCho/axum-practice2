// JSON 형태로 응답하기 위해서 axum의 Json 구조체를 사용한다.
use axum::Json;
// mod.rs에 정의한 요소들을 사용하고자 할 때는 use문을 쓰나 가장 먼저 crate 에서 시작해야한다.
use crate::dashboard::structs::{Dashboard, LikeContent};

pub async fn load_dashboard() -> Json<Dashboard> {

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

    // Rust의 구조체를 JSON으로 직렬화하여 반환한다.
    Json(dashboard)
}