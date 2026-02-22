use axum::{
    Extension, Json, extract::State, http::{
        HeaderMap, 
        HeaderValue, 
        StatusCode, 
        header
    }, response::IntoResponse};
use bcrypt::{DEFAULT_COST, hash, verify};
use serde::{Deserialize, Serialize};

use crate::{JWTConfigState, auth::jwt::create_token};

#[derive(Deserialize, Clone, Debug)]
pub struct LoginParameter {
    pub account: String,
    pub password: String
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct LoginResult {
    message: String,
    nickname: String
}



#[axum::debug_handler] 
pub async fn login(State(jwt_config_state): State<JWTConfigState>, Json(payload): Json<LoginParameter>) -> impl IntoResponse {
    // 이 곳에 로그인 로직이 들어감
    // 아직 DB 연결을 하지 않았으니 하드코딩된 값과 비교하도록 한다.

    let fake_account = "test111";
    let fake_password = "qwerty";

    let account = payload.account;
    let password = payload.password;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));

    match account == fake_account {
        true => {
            let hashed = hash(password, DEFAULT_COST).unwrap();
            let verified = verify(fake_password, &hashed).unwrap();
            match verified {
                true => {
                let token = create_token(account, jwt_config_state.jwt_config).await;
                let login_result = LoginResult {
                    message: "Login Success".to_string(),
                    nickname: "shandy".to_string()
                };

                headers.insert(header::AUTHORIZATION, HeaderValue::from_str(&token).unwrap());
                (
                    StatusCode::OK,
                    headers,
                    Extension(format!("{:?}", login_result.clone())),
                    Json(login_result.clone())
                )
                }
                false => {
                    let login_result = LoginResult {
                        message: "Account or Password isn't validate".to_string(),
                        nickname: "".to_string()
                    };
                    (
                        StatusCode::UNAUTHORIZED,
                        headers,
                        Extension(format!("{:?}", login_result.clone())),
                        Json(login_result.clone())
                    )
                }
            }
        },
        false => {
            let login_result = LoginResult {
                message: "Account or Password isn't validate".to_string(),
                nickname: "".to_string()
            };
            (
                StatusCode::UNAUTHORIZED,
                headers,
                Extension(format!("{:?}", login_result.clone())),
                Json(login_result.clone())
            )
        }
    }
}

