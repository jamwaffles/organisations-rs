use actix_web::{HttpRequest, HttpResponse};
use responses::{HealthcheckSuccess, SuccessfulResponse};

pub fn health(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(SuccessfulResponse {
        result: HealthcheckSuccess::new(),
    })
}
