use events::{Event, MembershipRole, MembershipStatus, OrganisationId, OrganisationType};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct Membership {
    pub user_id: Uuid,
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

pub fn hydrate(events: &Vec<Event>) -> Membership {
    events
        .iter()
        .cloned()
        .fold(Membership::default(), |acc, event| match event {
            Event::AccountInvitedToOrg(ev) => Membership {
                organisation_id: ev.organisation_id,
                name: ev.name,
                email: ev.email,
                invite_token: ev.invite_token,
                organisation_type: ev.organisation_type,
                membership_role: ev.membership_role,

                membership_status: MembershipStatus::Pending,
                ..acc
            },
            Event::AccountInviteToOrgAccepted(ev) => Membership {
                user_id: ev.user_id,
                organisation_id: ev.organisation_id,
                organisation_type: ev.organisation_type,
                invite_token: ev.invite_token,

                membership_status: MembershipStatus::Accepted,
                ..acc
            },
            Event::AccountInviteToOrgRevoked(ev) => Membership {
                user_id: ev.user_id,
                organisation_id: ev.organisation_id,

                membership_status: MembershipStatus::Revoked,
                ..acc
            },
            Event::MembershipEdited(ev) => Membership {
                user_id: ev.user_id,
                organisation_id: ev.organisation_id,
                membership_role: ev.membership_role,

                ..acc
            },
            _ => unimplemented!(),
        })
}
