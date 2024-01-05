use crate::model::{
    common_model::ResObj,
    user_model::{LoginParams, LoginRes, User},
};

use crate::utils::res::{res_json_ok, Res};
use salvo::oapi::{endpoint, extract::JsonBody};

/// 登录
#[endpoint(
    operation_id = String::from("login"),
    tags("user"),
    responses(
        (status_code = 200,body=ResObj<LoginRes>,description ="登录")
    ),
  )]
pub async fn login(_login_body: JsonBody<LoginParams>) -> Res<LoginRes> {
    let user = User {
        name: "小明".to_string(),
    };
    let login_res = LoginRes {
        token: "token".to_string(),
        user,
    };

    Ok(res_json_ok(Some(login_res)))
}
