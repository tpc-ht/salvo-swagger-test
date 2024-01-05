use crate::controller::swagger_controller;
use salvo::logging::Logger;
use salvo::oapi::OpenApi;
use salvo::prelude::SwaggerUi;
use salvo::prelude::{CatchPanic, SessionHandler};
use salvo::session::CookieStore;
use salvo::Router;

pub mod user_router;

pub fn init_router() -> Router {
    // 服务器静态资源
    let router = Router::new()
        .path("/api")
        .hoop(Logger::new()) // 日志中间件
        .hoop(CatchPanic::new())
        .push(user_router::init_router_no_token());

    let session_handler = SessionHandler::builder(
        CookieStore::new(),
        b"salvo-adminsalvo-adminalvo-adminsalvo-admin2023salvo-admin2023salvo-admin2023",
    )
    .build()
    .unwrap();
    let doc = OpenApi::new("后台接口文档", "0.0.1")
        .tags(["user"])
        .merge_router(&router);
    let router = router.push(
        Router::new().hoop(session_handler).push(
            Router::new()
                .hoop(swagger_controller::auth_token)
                .push(doc.into_router("/api-doc/openapi.json"))
                .push(SwaggerUi::new("/api/api-doc/openapi.json").into_router("swagger-ui")),
        ),
    );

    router
}
