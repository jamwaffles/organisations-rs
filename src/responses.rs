use serde::Serialize;

use aggregators::members_by_organisation_id;

#[derive(Serialize)]
pub struct SuccessfulResponse<T: Serialize> {
    pub result: T,
}

#[derive(Serialize)]
pub struct HealthcheckSuccess {
    ok: bool,
}

impl HealthcheckSuccess {
    pub fn new() -> Self {
        Self { ok: true }
    }
}

#[derive(Serialize)]
pub struct GetOrganisationMembersSuccess {
    users: Vec<members_by_organisation_id::Membership>,
}

impl GetOrganisationMembersSuccess {
    pub fn new(users: Vec<members_by_organisation_id::Membership>) -> Self {
        Self { users }
    }
}
