mod get_organisation_members;
mod update_membership;

use actix_web::{HttpRequest, HttpResponse};
use responses::{HealthcheckSuccess, SuccessfulResponse};
use AppState;

pub fn health(_req: &HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::Ok().json(SuccessfulResponse {
        result: HealthcheckSuccess::new(),
    })
}

pub use self::get_organisation_members::get_organisation_members;
pub use self::update_membership::update_membership;
