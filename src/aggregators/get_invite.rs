use events::{
    AccountInvitedToOrg,
    AccountInviteToOrgAccpted,
    AccountInviteToOrgRevoked,
    MembershipEdited,
    Membership,
    Event,
};

#[derive(Debug)]
pub struct Membership {
    user_id: String,
    organisation_id: String,
    organisation_type: String,
    name: String,
    email: String,
    invite_token: String,
    membership_status: String,
    membership_role: String,
}

impl Default for Membership {
    fn default() -> Membership {
        Membership {
            user_id: "".into(),
            organisation_id: "".into(),
            organisation_type: "".into(),
            name: "".into(),
            email: "".into(),
            invite_token: "".into(),
            membership_status: "NOT_INVITED".into(),
            membership_role: "NOT_INVITED".into(),
        }
    }
}

pub fn hydrate(events: &Vec<Event>) -> Membership {
    events.iter().fold(
        Membership::default(),
        |acc, event| {
            match event {
                Event::AccountInvitedToOrg(ev) => {
                    Membership {
                        organisation_id: ev.organisation_id,
                        name: ev.name,
                        email: ev.email,
                        invite_token: ev.invite_token,
                        organisation_type, ev.organisation_type,
                        membership_role: ev.membership_role,

                        membership_status: "PENDING",
                        ..acc
                    }
                }
                Event::AccountInviteToOrgAccepted(ev) => {
                    Membership {
                        user_id: ev.user_id,
                        organisation_id: ev.organisation_id,
                        organisation_type: ev.organisation_type,
                        invite_token: ev.invite_token,
                        
                        membership_status: "ACCEPTED",
                        ..acc
                    }
                }
                Event::AccountInviteToOrgRevoked(ev) => {
                    Membership {
                        user_id: ev.user_id,
                        organisation_id: ev.organisation_id,
                        
                        membership_status: "REVOKED",
                        ..acc
                    }
                }
                Event::MembershipEdited(ev) => {
                    Membership {
                        user_id: ev.user_id,
                        organisation_id: ev.organisation_id,
                        membership_role: ev.membership_role,
                        
                        ..acc,
                    }
                }
                _ => acc
        }
    )}
}
