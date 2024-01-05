use crate::controller::user_controller;
use salvo::Router;

pub fn init_router_no_token() -> Router {
    Router::new().push(
        // 登录
        Router::with_path("/login").post(user_controller::login),
    )
}
