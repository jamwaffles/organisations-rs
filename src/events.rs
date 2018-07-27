use uuid::Uuid;

pub type OrganisationId = Uuid;
pub type UserId = Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub enum MembershipRole {
    #[serde(rename = "BASIC")]
    Basic,
    #[serde(rename = "ADMIN")]
    Admin,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum OrganisationType {
    #[serde(rename = "pdx-vendor")]
    PdxVendor,
    #[serde(rename = "pdx-consumer")]
    PdxConsumer,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountInvitedToOrg {
    name: String,
    email: String,
    organisation_id: OrganisationId,
    invite_token: String,
    organisation_type: OrganisationType,
    membership_role: MembershipRole,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountInviteToOrgAccpted {
    user_id: UserId,
    organisation_type: OrganisationType,
    organisation_id: OrganisationId,
    invite_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountInviteToOrgRevoked {
    user_id: UserId,
    invite_token: String,
    organisation_id: OrganisationId,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MembershipEdited {
    user_id: UserId,
    organisation_id: OrganisationId,
    membership_role: MembershipRole,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountUpdated {
    user_id: UserId,
    name: Option<String>,
    bio: Option<String>,
    interests: Option<Vec<String>>,
    location: Option<String>,
    work_organisation: Option<String>,
    work_role: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AvatarEdited {
    user_id: UserId,
    avatar: String,
    avatar_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Event {
    AccountInvitedToOrg(AccountInvitedToOrg),
    AccountInviteToOrgAccpted(AccountInviteToOrgAccpted),
    AccountInviteToOrgRevoked(AccountInviteToOrgRevoked),
    MembershipEdited(MembershipEdited),
}
