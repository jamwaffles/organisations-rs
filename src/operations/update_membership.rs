
use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Path, State};
use eventstore::GetOrganisationMembersQuery;
use futures::future::Future;
use responses::SuccessfulResponse;
use uuid::Uuid;
use AppState;

use aggregators::members_by_organisation_id::hydrate as hydrate_members;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMembershipCmd {
    organisation_id: Uuid,
    user_id: Uuid,
    name: Option<String>,
    email: Option<String>,
}

pub fn update_membership(
    (payload, state): (Json<UpdateMembershipCmd>, State<AppState>),
) -> FutureResponse<HttpResponse> {

    // Store an event ¯\_(ツ)_/¯

    hydrate_members(
        &state,
        GetOrganisationMembersQuery {
            organisation_id: organisation_id.into_inner(),
        },
    ).and_then(|members| Ok(HttpResponse::Ok().json(SuccessfulResponse { result: members })))
    .responder()
}
