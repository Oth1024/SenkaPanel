use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginUserRequest {
    pub user_name: String,
    pub passwd_hash: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginUserResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterUserRequest {
    pub email_addr: String,
    pub user_name: String,
    pub passwd_hash: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterUserResponse {
    pub success: bool,
    pub message: String,
}
