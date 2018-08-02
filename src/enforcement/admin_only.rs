use actix_web::middleware::{Middleware, Started};
use actix_web::{FromRequest, HttpRequest, HttpResponse, Result};
use events::MembershipRole;
use jwt::CurrentAuth;

pub struct AdminOnly;

impl<S> Middleware<S> for AdminOnly {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        CurrentAuth::extract(req).map(|token| {
            match token
                .memberships
                .iter()
                .find(|membership| membership.membership_role == MembershipRole::Admin)
            {
                Some(_) => Started::Done,
                None => Started::Response(HttpResponse::Unauthorized().finish()),
            }
        })
    }
}
