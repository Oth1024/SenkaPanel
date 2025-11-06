use rocket::post;
use rocket::serde::json::Json;

use crate::protocol::user::{RegisterUserRequest, RegisterUserResponse};

#[post("/api/user_register", data = "<register_user_request>")]
pub fn post_register(register_user_request: Json<RegisterUserRequest>) -> String {
    let json_content = register_user_request.0;

    // TODO:转换为合法字符串
    // TODO:验证账户信息是否合法
    // TODO:验证账户是否存在的逻辑
    // TODO:转换为sql实体

    serde_json::to_string(&RegisterUserResponse {
        success: true,
        message: String::from(""),
    })
    .unwrap()
}
