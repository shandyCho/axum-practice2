use std::path::Path;

use serde::Deserialize;
use tokio::fs;

#[derive(Deserialize)]
pub struct Config {
    jwt: JwtConfig,
    axum_config: AxumConfig,
    database: DatabaseConfig
}

impl Config {
    pub fn get_jwt_config(&self) -> JwtConfig {
        let jwt = &self.jwt;
        jwt.clone()
    }
    pub fn get_axum_config(&self) -> AxumConfig {
        let axum_config = &self.axum_config;
        axum_config.clone()
    }
    pub fn get_database_config(&self) -> DatabaseConfig {
        let database_config = &self.database;
        database_config.clone()
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct JwtConfig {
    aud: String,
    iss: String,
    sub: String,
    secret: String
}

impl JwtConfig {
        pub fn get_aud(&self) -> String {
        let aud = &self.aud;
        aud.to_string()
    }
    pub fn get_iss(&self) -> String {
        let iss = &self.iss;
        iss.to_string()
    }
    pub fn get_sub(&self) -> String {
        let sub = &self.sub;
        sub.to_string()
    }
    pub fn get_secret(&self) -> String {
        let secret = &self.secret;
        secret.to_string()
    }
}

#[derive(Deserialize, Clone)]
pub struct AxumConfig {
    addr: String
}

impl AxumConfig {
    pub fn get_addr(&self) -> &String {
        &self.addr
    }
}

#[derive(Deserialize, Clone)]
pub struct DatabaseConfig {
    url: String,
    user: String,
    password: String
}

pub async fn get_application_config() -> Config {
    // cargo run 했을 때의 프로젝트 루트 경로 가져오기
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(manifest_dir).join("resource/application_config/config.toml");
    let application_config_string = fs::read_to_string(path)
        .await.expect("Fail to find application_config.toml");
    
    let config = toml::from_str(&application_config_string)
        .expect("Fail to parsing string to struct. Check your config");
    config
}