use rocket::post;
use rocket::serde::json::Json;

use crate::protocol::user::{LoginUserRequest, LoginUserResponse};

#[post("/api/user_login", data = "<login_user_info_request>")]
pub fn post_login(login_user_info_request: Json<LoginUserRequest>) -> String {
    let json_content = login_user_info_request.0;

    // TODO:转换为合法字符串
    // TODO:验证账户是否存在的逻辑
    serde_json::to_string(&LoginUserResponse {
        success: true,
        message: String::from(""),
    })
    .unwrap()
}