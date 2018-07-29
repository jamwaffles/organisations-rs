use serde::Serialize;

use aggregators::get_invite::{hydrate as hydrate_membership, Membership};

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
pub struct GetOrganisationUsersSuccess {
    users: Vec<Membership>,
}

impl GetOrganisationUsersSuccess {
    pub fn new() -> Self {
        unimplemented!();
    }
}
