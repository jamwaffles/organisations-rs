use actix_web::http::Method;
use actix_web::middleware::{Middleware, Started};
use actix_web::{FromRequest, HttpRequest, HttpResponse, Json, Path, Result};
use events::MembershipStatus;
use futures::Future;
use jwt::CurrentAuth;
use uuid::Uuid;

pub struct OrganisationMember;

#[derive(Debug, Deserialize, Serialize)]
struct OrganisationIdPayload {
    organisation_id: Uuid,
}

impl<S: 'static> Middleware<S> for OrganisationMember {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        let token = CurrentAuth::extract(req);

        // Either get req_org_id from query or post body
        let req_organisation_id = match req.method() {
            &Method::GET => Path::<Uuid>::extract(req).unwrap().into_inner(),
            &Method::POST => {
                Json::<OrganisationIdPayload>::extract(&req.clone())
                    .wait()
                    .unwrap()
                    .organisation_id
            }
            _ => unreachable!(), // TODO: return a 500
        };
        // TODO: Properly handle unwraps

        // Auth on presence of token
        // TODO: Validate contents of token
        match token {
            Ok(token) => token
                .memberships
                .iter()
                .find(|membership| {
                    membership.organisation_id == req_organisation_id
                        && membership.membership_status == MembershipStatus::Accepted
                }).map(|_| Ok(Started::Done))
                .unwrap_or(Ok(Started::Response(HttpResponse::Unauthorized().finish()))),
            Err(_) => Ok(Started::Response(HttpResponse::Unauthorized().finish())),
        }
    }
}
