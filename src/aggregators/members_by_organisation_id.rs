use actix_web::Error;
use events::{MembershipRole, MembershipStatus, OrganisationId, OrganisationType, UserId};
use eventstore::{EventsQuery, GetOrganisationMembersQuery};
use futures::{future, Future};
use uuid::Uuid;
use AppState;

#[derive(Clone, Debug, Serialize)]
pub struct Membership {
    pub user_id: UserId,
    pub organisation_id: OrganisationId,
    pub organisation_type: OrganisationType,
    pub name: String,
    pub email: String,
    pub invite_token: String,
    pub membership_status: MembershipStatus,
    pub membership_role: MembershipRole,
}

impl Default for Membership {
    fn default() -> Membership {
        Membership {
            user_id: Uuid::new_v4(),
            organisation_id: Uuid::new_v4(),
            organisation_type: OrganisationType::PdxVendor,
            name: "".into(),
            email: "".into(),
            invite_token: "".into(),
            membership_status: MembershipStatus::NotInvited,
            membership_role: MembershipRole::Basic,
        }
    }
}

pub fn hydrate(
    state: &AppState,
    query: GetOrganisationMembersQuery,
) -> impl Future<Item = Vec<Membership>, Error = Error> {
    state
        .eventstore
        .send::<EventsQuery>(query.into())
        .from_err()
        .and_then(|_events| future::ok(vec![Membership::default()]))
}
