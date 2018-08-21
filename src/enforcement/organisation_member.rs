use actix_web::error::ErrorUnauthorized;
use actix_web::Error as WebError;
use extractors::jwt::CurrentAuth;
use futures::future::{err as FutErr, ok as FutOk, Future};
use uuid::Uuid;

pub fn is_organisation_member(
    auth: &CurrentAuth,
    organisation_id: Uuid,
) -> impl Future<Item = (), Error = WebError> {
    auth.memberships
        .iter()
        .find(|membership| membership.organisation_id == organisation_id)
        .map(|_| FutOk(()))
        .unwrap_or(FutErr(ErrorUnauthorized("Nope")))
}
