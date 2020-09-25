use crate::web::RequestContext;
use actix_web::HttpResponse;
use crate::templates::static_pages::index::LandingPage;
use crate::templates::StaticPage;
use crate::templates::with_alert::WithAlert;

/// Log a user out.
pub async fn logout_service(req_ctx: RequestContext) -> HttpResponse {
    let identity = req_ctx.identity();
    identity.forget();

    HttpResponse::Ok()
        .body(WithAlert::render_into_page(
            &req_ctx,
            LandingPage::PAGE_TITLE,
            "success",
            "You are logged out",
            &LandingPage
        ))
}