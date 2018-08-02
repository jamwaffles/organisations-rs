use actix_web::http::Method;
use actix_web::middleware::{Middleware, Started};
use actix_web::{FromRequest, HttpRequest, HttpResponse, Json, Path, Result};
use events::MembershipStatus;
use futures::future::ok;
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
        // Either get req_org_id from query or post body
        let req_organisation_id = match req.method() {
            &Method::GET => Some(Path::<Uuid>::extract(req).unwrap().into_inner()),
            &Method::POST => {
                println!("finding org_id");
                let res = Json::<OrganisationIdPayload>::extract(&req.clone())
                    .map_err(|E| {
                        println!("Error: {}", E.to_string());
                        E
                    }).and_then(|w| {
                        println!("w {:?}", w);
                        ok(())
                    });
                println!("Returned None");
                Some(Uuid::new_v4())
            }
            _ => None,
        };
        // TODO: Properly handle unwraps
        println!("ORGANISATION_ID: {}", req_organisation_id.unwrap());

        let token = CurrentAuth::extract(req);
        println!("found token, {:?}", token); // We know it reaches this point DEBUG

        // Auth on presence of token
        // TODO: Validate contents of token
        match token {
            Ok(token) => token
                .memberships
                .iter()
                .find(|membership| {
                    membership.organisation_id == req_organisation_id.unwrap()
                        && membership.membership_status == MembershipStatus::Accepted
                }).map(|_| Ok(Started::Done))
                .unwrap_or(Ok(Started::Response(HttpResponse::Unauthorized().finish()))),
            Err(_) => Ok(Started::Response(HttpResponse::Unauthorized().finish())),
        }
    }
}
