#[derive(Debug, Deserialize, Serialize)]
pub struct AccountInvitedToOrg {
    name: String,
    email: String,
    organisation_id: String,
    invite_token: String,
    organisation_type: String,
    membership_role: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountInviteToOrgAccpted {
    user_id: String,
    organisation_type: String,
    organisation_id: String,
    invite_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountInviteToOrgRevoked {
    user_id: String,
    invite_token: String,
    organisation_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MembershipEdited {
    user_id: String,
    organisation_id: String,
    membership_role: String,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Event {
    AccountInvitedToOrg(AccountInvitedToOrg),
    AccountInviteToOrgAccpted(AccountInviteToOrgAccpted),
    AccountInviteToOrgRevoked(AccountInviteToOrgRevoked),
    MembershipEdited(MembershipEdited),
}
