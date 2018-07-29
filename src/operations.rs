use actix_web::{HttpRequest, HttpResponse, Path};
use responses::{HealthcheckSuccess, SuccessfulResponse};
use uuid::Uuid;

pub fn health(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json(SuccessfulResponse {
        result: HealthcheckSuccess::new(),
    })
}

pub fn get_organisation_members(organisation_id: Path<Uuid>) -> HttpResponse {
    HttpResponse::Ok().json(SuccessfulResponse {
        result: format!("Org ID: {}", organisation_id),
    })
}
