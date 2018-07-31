use actix_web::Error;
use events::{Event, MembershipRole, MembershipStatus, OrganisationId, OrganisationType, UserId};
use eventstore::{EventsQuery, GetOrganisationMembersQuery};
use futures::{future, Future};
use std::collections::HashMap;
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
        .and_then(move |events| {
            let members = events.unwrap().iter().fold(
                HashMap::new(),
                |mut acc: HashMap<Uuid, Membership>, event: &Event| {
                    let event_user_id = match event {
                        Event::AccountInviteToOrgAccepted(ev) => Some(ev.user_id),
                        Event::AccountInviteToOrgRevoked(ev) => Some(ev.user_id),
                        Event::MembershipEdited(ev) => Some(ev.user_id),
                        _ => None,
                    };

                    if let Some(user_id) = event_user_id {
                        let default = Membership::default();
                        let existing = acc.get(&user_id).unwrap_or(&default).clone();

                        let updated = match event {
                            Event::AccountInviteToOrgAccepted(ev) => Membership {
                                user_id: ev.user_id,
                                organisation_id: ev.organisation_id,
                                organisation_type: ev.organisation_type.clone(),
                                invite_token: ev.invite_token.clone(),

                                membership_status: MembershipStatus::Accepted,
                                ..existing.clone()
                            },
                            Event::AccountInviteToOrgRevoked(ev) => Membership {
                                user_id: ev.user_id,
                                organisation_id: ev.organisation_id,

                                membership_status: MembershipStatus::Revoked,
                                ..existing.clone()
                            },
                            Event::MembershipEdited(ev) => Membership {
                                user_id: ev.user_id,
                                organisation_id: ev.organisation_id,
                                membership_role: ev.membership_role.clone(),

                                ..existing.clone()
                            },
                            _ => existing.clone(),
                        };

                        acc.insert(user_id, updated);
                    }

                    acc
                },
            );

            future::ok(members.values().map(|v| v.clone()).collect())
        })
}
