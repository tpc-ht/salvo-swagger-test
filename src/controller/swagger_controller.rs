use salvo::prelude::{Redirect, SessionDepotExt};
use salvo::session::Session;
use salvo::{handler, Depot, FlowCtrl, Request, Response};

#[handler]
pub async fn auth_token(
    req: &mut Request,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
    depot: &mut Depot,
) {
    ctrl.call_next(req, depot, res).await;
}

#[handler]
pub async fn swagger_login(_req: &mut Request, res: &mut Response, depot: &mut Depot) {
    let mut session = Session::new();
    session.insert("swagger-auth", "salvo-admin").unwrap();
    depot.set_session(session);
    res.render(Redirect::other("/swagger-ui"));
}
