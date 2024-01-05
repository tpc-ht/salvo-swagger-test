use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
#[salvo(schema(rename_all = "camelCase"))]
pub struct CaptchaRes {
    #[serde(rename = "captchaEnabled")]
    pub captcha_enabled: Option<bool>,
    pub img: String,
    pub uuid: String,
}

// 登录请求参数
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct LoginParams {
    pub code: String,
    pub username: String,
    pub password: String,
}

// 登录返回
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct LoginRes {
    pub token: String,
    pub user: User,
}

// 用户信息
#[derive(Serialize, Deserialize, Clone, Debug, ToSchema)]
pub struct User {
    pub name: String,
}
