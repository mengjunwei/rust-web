use actix_web::{dev::HttpServiceFactory, web};
use services::public::HealthRouter;

/// 注册路由
///
/// Service Hub Module: [`service_hub`]
pub fn register() -> impl HttpServiceFactory {
    web::scope("/api/v1").service(HealthRouter::register())
}
