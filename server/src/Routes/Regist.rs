use rocket::post;
use rocket::serde::json::Json;

use crate::protocol::user::{RegistUserRequest, RegistUserResponse};

#[post("/api/user_regist", data = "<regist_user_request>")]
pub fn post_regist(regist_user_request: Json<RegistUserRequest>) -> String {
    let json_content = regist_user_request.0;

    // TODO:转换为合法字符串
    // TODO:验证账户信息是否合法
    // TODO:验证账户是否存在的逻辑
    // TODO:转换为sql实体

    serde_json::to_string(&RegistUserResponse {
        success: true,
        message: String::from(""),
    })
    .unwrap()
}
