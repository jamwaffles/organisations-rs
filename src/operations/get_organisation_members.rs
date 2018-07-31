use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Path, State};
use eventstore::GetOrganisationMembersQuery;
use futures::future::Future;
use responses::SuccessfulResponse;
use uuid::Uuid;
use AppState;

use aggregators::members_by_organisation_id::hydrate as hydrate_members;

pub fn get_organisation_members(
    (organisation_id, state): (Path<Uuid>, State<AppState>),
) -> FutureResponse<HttpResponse> {
    hydrate_members(
        &state,
        GetOrganisationMembersQuery {
            organisation_id: organisation_id.into_inner(),
        },
    ).and_then(|members| Ok(HttpResponse::Ok().json(SuccessfulResponse { result: members })))
    .responder()
}
