use actix_web::Responder;
use response::Response;

pub struct HealthController;

impl HealthController {
    /// 健康检查
    pub async fn health() -> impl Responder {
        Response::ok().data("ok")
    }
}
