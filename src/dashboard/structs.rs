// serde 는 Rust의 데이터 직렬화 및 역직렬화를 위한 라이브러리로, JSON과 같은 형식으로 데이터를 변환하는 데 사용된다.
// Serialize 트레이트는 Rust 구조체를 JSON으로 변환할 수 있게 해준다.
// Deserialize 트레이트는 JSON 데이터를 Rust 구조체로 변환할 수 있게 해준다.
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LikeContent {
    content_title: String,
    content_description: String,
    content_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dashboard {
    name: String,
    description: String,
    like_list: Vec<LikeContent>
}

// 내부 필드 접근을 직접 하지 않도록 new 메소드를 통해 생성자를 구현한다.
impl Dashboard {
    pub fn new(name: String, description: String, like_list: Vec<LikeContent>) -> Self {
        Dashboard {
            name,
            description,
            like_list,
        }
    }
}

impl LikeContent {
    pub fn new(content_title: String, content_description: String, content_id: u32) -> Self {
        LikeContent {
            content_title,
            content_description,
            content_id,
        }
    }
}