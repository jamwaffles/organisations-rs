use actix_web::middleware::{Middleware, Started};
use actix_web::{HttpRequest, HttpResponse, Result};
use events::MembershipRole;
use middleware::CurrentAuth;

pub struct AdminOnly;

impl<S> Middleware<S> for AdminOnly {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        let exts = req.extensions();
        let token = exts.get::<CurrentAuth>();

        // Auth on presence of token
        match token {
            Some(token) => token
                .memberships
                .iter()
                .find(|membership| membership.membership_role == MembershipRole::Admin)
                .map(|_| Ok(Started::Done))
                .unwrap_or(Ok(Started::Response(HttpResponse::Unauthorized().finish()))),
            None => Ok(Started::Response(HttpResponse::Unauthorized().finish())),
        }
    }
}
