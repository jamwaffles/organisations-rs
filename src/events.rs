use uuid::Uuid;

pub type OrganisationId = Uuid;
pub type UserId = Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MembershipRole {
    #[serde(rename = "BASIC")]
    Basic,
    #[serde(rename = "ADMIN")]
    Admin,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum OrganisationType {
    #[serde(rename = "pdx-vendor")]
    PdxVendor,
    #[serde(rename = "pdx-consumer")]
    PdxConsumer,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccountInvitedToOrg {
    pub name: String,
    pub email: String,
    pub organisation_id: OrganisationId,
    pub invite_token: String,
    pub organisation_type: OrganisationType,
    pub membership_role: MembershipRole,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccountInviteToOrgAccepted {
    pub user_id: UserId,
    pub organisation_type: OrganisationType,
    pub organisation_id: OrganisationId,
    pub invite_token: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccountInviteToOrgRevoked {
    pub user_id: UserId,
    pub invite_token: String,
    pub organisation_id: OrganisationId,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MembershipEdited {
    pub user_id: UserId,
    pub organisation_id: OrganisationId,
    pub membership_role: MembershipRole,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccountUpdated {
    pub user_id: UserId,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub interests: Option<Vec<String>>,
    pub location: Option<String>,
    pub work_organisation: Option<String>,
    pub work_role: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AvatarEdited {
    pub user_id: UserId,
    pub organisation_id: OrganisationId,
    pub membership_role: MembershipRole,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum Event {
    AccountInvitedToOrg(AccountInvitedToOrg),
    AccountInviteToOrgAccepted(AccountInviteToOrgAccepted),
    AccountInviteToOrgRevoked(AccountInviteToOrgRevoked),
    MembershipEdited(MembershipEdited),
}
