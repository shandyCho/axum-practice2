use axum::{extract::{Request, State}, http::{StatusCode, header}, middleware::Next, response::Response};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind};
use serde::{Deserialize, Serialize};

use crate::{JWTConfigState, config::application_config::JwtConfig};


#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,         // Optional. Audience  - JWT의 수신자를 지정합니다.
    exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)  - JWT가 처리 대상으로 수락되어서는 안 되는 만료 시간을 지정합니다.
    // iat: usize,          // Optional. Issued at (as UTC timestamp)  - JWT가 발행된 시간을 나타냅니다.
    iss: String,         // Optional. Issuer    - JWT를 발급한 주체를 식별합니다. 예를 들어 조직 이름이나 웹사이트 URL 등이 있습니다.
    // nbf: usize,          // Optional. Not Before (as UTC timestamp)  - JWT 처리를 위한 수락 시작 시간을 지정합니다
    sub: String,         // Optional. Subject (whom token refers to)    - JWT의 주체를 식별합니다(예: 사용자 이름 또는 계좌 번호).
    account: String      // 사용자 계정 (추가 필드)
}

pub async fn verify_jwt(State(jwt_config_state): State<JWTConfigState>, req: Request, next: Next) -> Result<Response, StatusCode> {
    println!("this is verify_jwt");
    // 여기서 Authorization 헤더 안에 있는 데이터를 추출함
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 여기서 JWT 검증 들어가야함
    let jwt_config = jwt_config_state.jwt_config;
    let mut validation = Validation::new(Algorithm::HS256);
    validation.sub = Some(jwt_config.get_sub());
    validation.set_audience(&[jwt_config.get_aud()]);
    validation.set_issuer(&[jwt_config.get_iss()]);
    validation.set_required_spec_claims(&["exp", "sub", "aud"]);

    let token_data = match decode::<Claims>(auth_header, &DecodingKey::from_secret(jwt_config.get_secret().as_bytes()), &validation) {
        Ok(c) => Ok(c),
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => Err(StatusCode::UNAUTHORIZED), // Example on how to handle a specific error
            ErrorKind::InvalidIssuer => Err(StatusCode::UNAUTHORIZED), // Example on how to handle a specific error
            _ => Err(StatusCode::UNAUTHORIZED),
        },
    };

    match token_data {
        Ok(c) => {
            let response = next.run(req).await;
            Ok(response)
        }
        Err(err) => Err(err)
    }
    // Some(token_data)
}

pub async fn create_token(account: String, jwt_config: JwtConfig) -> String {
    let claims = Claims{
        aud: jwt_config.get_aud(),
        exp: 60 * 60,
        iss: jwt_config.get_iss(),
        sub: jwt_config.get_sub(),
        account: account
    };

    let secret = jwt_config.get_secret();

    match encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())) {
        Ok(t) => t,
        Err(e) => e.to_string()
    }
}


